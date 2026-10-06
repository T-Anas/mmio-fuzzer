//! Interrupt injection: SysTick, the NVIC and a small MMIO router.
//!
//! Firmware that is driven by interrupts needs more than a bus: it needs a
//! tick timer that raises SysTick, an NVIC that can hold peripheral IRQs
//! pending, and an exception return path so handlers give control back. This
//! module models the first two; the interpreter handles the third.

use std::cell::RefCell;
use std::rc::Rc;

use mmio_core::{AccessWidth, PhysAddr};
use mmio_emu::memory::MmioHandler;

use crate::peripheral::PeripheralModel;

/// Base of the private peripheral bus system region.
pub const SYSTEM_BASE: u32 = 0xE000_E000;
/// Exclusive end of the region the system controller owns.
pub const SYSTEM_END: u32 = 0xE000_F000;

const SYSTICK_CTRL: u32 = 0x010;
const SYSTICK_LOAD: u32 = 0x014;
const SYSTICK_VAL: u32 = 0x018;
const SYSTICK_CALIB: u32 = 0x01C;

const NVIC_ISER: u32 = 0x100;
const NVIC_ICER: u32 = 0x180;
const NVIC_ISPR: u32 = 0x200;
const NVIC_ICPR: u32 = 0x280;
const SCB_ICSR: u32 = 0xD04;
const SCB_VTOR: u32 = 0xD08;

const SYSTICK_ENABLE: u32 = 1 << 0;
const SYSTICK_TICKINT: u32 = 1 << 1;
const SYSTICK_COUNTFLAG: u32 = 1 << 16;

/// SysTick exception number.
pub const SYSTICK_EXCEPTION: u16 = 15;
/// First external interrupt exception number (IRQ0).
pub const IRQ_EXCEPTION_BASE: u16 = 16;

/// Shared system-controller state.
#[derive(Clone, Debug, Default)]
pub struct SystemState {
    pub systick_ctrl: u32,
    pub systick_load: u32,
    pub systick_val: u32,
    pub systick_calib: u32,
    /// Enabled external IRQs (bit *n* is IRQ *n*).
    pub enabled: u32,
    /// Pending external IRQs.
    pub pending: u32,
    pub vtor: u32,
    systick_pending: bool,
    pub ticks: u64,
}

impl SystemState {
    /// Advances the SysTick counter by one.
    pub fn tick(&mut self) {
        self.ticks += 1;
        if self.systick_ctrl & SYSTICK_ENABLE == 0 {
            return;
        }
        if self.systick_val == 0 {
            self.systick_val = self.systick_load & 0x00FF_FFFF;
            if self.systick_val == 0 {
                self.raise_systick();
                return;
            }
        }
        self.systick_val -= 1;
        if self.systick_val == 0 {
            self.raise_systick();
        }
    }

    fn raise_systick(&mut self) {
        self.systick_ctrl |= SYSTICK_COUNTFLAG;
        if self.systick_ctrl & SYSTICK_TICKINT != 0 {
            self.systick_pending = true;
        }
    }

    /// Asserts an external IRQ line (e.g. a peripheral has data ready).
    pub fn assert_irq(&mut self, irq: u32) {
        self.pending |= 1 << irq;
    }

    pub fn systick_pending(&self) -> bool {
        self.systick_pending
    }

    /// The highest-priority exception ready to be taken, if any.
    ///
    /// `primask` is the core's PRIMASK: configurable exceptions are masked
    /// while it is set.
    pub fn next_exception(&self, primask: bool) -> Option<u16> {
        if primask {
            return None;
        }
        if self.systick_pending {
            return Some(SYSTICK_EXCEPTION);
        }
        let ready = self.pending & self.enabled;
        if ready == 0 {
            None
        } else {
            Some(IRQ_EXCEPTION_BASE + ready.trailing_zeros() as u16)
        }
    }

    /// Marks an exception as taken, clearing its pending state.
    pub fn mark_taken(&mut self, exception: u16) {
        if exception == SYSTICK_EXCEPTION {
            self.systick_pending = false;
        } else if (IRQ_EXCEPTION_BASE..IRQ_EXCEPTION_BASE + 32).contains(&exception) {
            self.pending &= !(1 << (exception - IRQ_EXCEPTION_BASE));
        }
    }
}

/// The MMIO view of [`SystemState`].
pub struct SystemControl {
    state: Rc<RefCell<SystemState>>,
}

impl Default for SystemControl {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemControl {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(SystemState::default())),
        }
    }

    pub fn state(&self) -> Rc<RefCell<SystemState>> {
        self.state.clone()
    }

    pub fn contains(&self, addr: u32) -> bool {
        (SYSTEM_BASE..SYSTEM_END).contains(&addr)
    }

    fn read_reg(&self, offset: u32, width: AccessWidth) -> u32 {
        let state = self.state.borrow();
        let value = match offset {
            SYSTICK_CTRL => state.systick_ctrl,
            SYSTICK_LOAD => state.systick_load,
            SYSTICK_VAL => state.systick_val,
            SYSTICK_CALIB => state.systick_calib,
            NVIC_ISER => state.enabled,
            NVIC_ISPR => state.pending,
            SCB_ICSR => {
                let mut value = 0;
                if state.systick_pending {
                    value |= 1 << 26; // PENDSTSET
                }
                if state.pending & state.enabled != 0 {
                    value |= 1 << 22; // VECTPENDING
                }
                value
            }
            SCB_VTOR => state.vtor,
            _ => 0,
        };
        value & width.mask()
    }

    fn write_reg(&self, offset: u32, value: u32) {
        let mut state = self.state.borrow_mut();
        match offset {
            SYSTICK_CTRL => state.systick_ctrl = value,
            SYSTICK_LOAD => state.systick_load = value & 0x00FF_FFFF,
            SYSTICK_VAL => state.systick_val = value & 0x00FF_FFFF,
            NVIC_ISER => state.enabled |= value,
            NVIC_ICER => state.enabled &= !value,
            NVIC_ISPR => state.pending |= value,
            NVIC_ICPR => state.pending &= !value,
            SCB_VTOR => state.vtor = value,
            _ => {}
        }
    }
}

impl MmioHandler for SystemControl {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Option<u32> {
        Some(self.read_reg(addr.raw().wrapping_sub(SYSTEM_BASE), width))
    }

    fn write(&mut self, addr: PhysAddr, _width: AccessWidth, value: u32) -> Option<()> {
        self.write_reg(addr.raw().wrapping_sub(SYSTEM_BASE), value);
        Some(())
    }
}

/// Routes the system region to the interrupt controller and everything else
/// to the peripheral model.
pub struct Router {
    system: SystemControl,
    inner: PeripheralModel,
}

impl Router {
    pub fn new(system: SystemControl, inner: PeripheralModel) -> Self {
        Self { system, inner }
    }
}

impl MmioHandler for Router {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Option<u32> {
        if self.system.contains(addr.raw()) {
            self.system.read(addr, width)
        } else {
            self.inner.read(addr, width)
        }
    }

    fn write(&mut self, addr: PhysAddr, width: AccessWidth, value: u32) -> Option<()> {
        if self.system.contains(addr.raw()) {
            self.system.write(addr, width, value)
        } else {
            self.inner.write(addr, width, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn systick_raises_after_load_ticks() {
        let control = SystemControl::new();
        let state = control.state();
        {
            let mut s = state.borrow_mut();
            s.systick_load = 5;
            s.systick_val = 5;
            s.systick_ctrl = SYSTICK_ENABLE | SYSTICK_TICKINT;
        }
        for _ in 0..4 {
            state.borrow_mut().tick();
            assert!(!state.borrow().systick_pending());
        }
        state.borrow_mut().tick();
        assert!(state.borrow().systick_pending());
        assert_eq!(
            state.borrow().next_exception(false),
            Some(SYSTICK_EXCEPTION)
        );
    }

    #[test]
    fn primask_masks_configurable_exceptions() {
        let control = SystemControl::new();
        let state = control.state();
        state.borrow_mut().assert_irq(3);
        state.borrow_mut().enabled = 1 << 3;
        assert_eq!(
            state.borrow().next_exception(false),
            Some(IRQ_EXCEPTION_BASE + 3)
        );
        assert_eq!(state.borrow().next_exception(true), None);
    }

    #[test]
    fn disabled_irq_is_not_taken() {
        let control = SystemControl::new();
        let state = control.state();
        state.borrow_mut().assert_irq(5);
        assert_eq!(state.borrow().next_exception(false), None);
        state.borrow_mut().enabled |= 1 << 5;
        assert_eq!(
            state.borrow().next_exception(false),
            Some(IRQ_EXCEPTION_BASE + 5)
        );
        state.borrow_mut().mark_taken(IRQ_EXCEPTION_BASE + 5);
        assert_eq!(state.borrow().next_exception(false), None);
    }

    #[test]
    fn register_access_round_trips() {
        let mut control = SystemControl::new();
        control.write(
            PhysAddr::new(SYSTEM_BASE + SYSTICK_LOAD),
            AccessWidth::Word,
            100,
        );
        assert_eq!(
            control.read(PhysAddr::new(SYSTEM_BASE + SYSTICK_LOAD), AccessWidth::Word),
            Some(100)
        );
    }
}
