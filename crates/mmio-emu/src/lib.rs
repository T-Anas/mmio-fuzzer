//! A small, honest interpreter for the ARMv6-M architecture.
//!
//! ARMv6-M is the instruction set of the Cortex-M0 and M0+. It is a *tiny*
//! architecture: 32-bit registers, Thumb only (no ARM state, no coprocessor),
//! and about fifty instructions plus a handful of 32-bit encodings. That makes
//! it a realistic target to implement faithfully in plain Rust, with no native
//! dependency, which in turn gives us total control over how MMIO traffic is
//! intercepted during fuzzing.
//!
//! The interpreter is deliberately split along the same lines as the spec:
//!
//! * [`regs`] holds the programmer's model (general registers, xPSR, control).
//! * [`memory`] is a [`mmio_core::Bus`] backed by a [`mmio_core::MemoryMap`].
//! * [`decode`] turns a fetched halfword or halfword pair into an [`decode::Inst`].
//! * [`exec`] performs one instruction.
//!
//! [`CortexM`] wires those pieces together.

pub mod decode;
pub mod exec;
pub mod memory;
pub mod regs;

pub use memory::FlatMemory;
pub use regs::{Cpu, Xpsr};

use mmio_core::{AccessWidth, Bus, CoreError, PhysAddr, Result};

/// A Cortex-M0 core executing from a 32-bit physical address space.
pub struct CortexM<B: Bus> {
    pub cpu: Cpu,
    pub bus: B,
    /// Number of instructions retired since construction.
    pub cycles: u64,
    /// If set, an exception is pending and will be taken before the next step.
    pending_exception: Option<u16>,
}

impl<B: Bus> CortexM<B> {
    pub fn new(cpu: Cpu, bus: B) -> Self {
        Self {
            cpu,
            bus,
            cycles: 0,
            pending_exception: None,
        }
    }

    pub fn pc(&self) -> PhysAddr {
        PhysAddr::new(self.cpu.pc())
    }

    pub fn set_pc(&mut self, pc: PhysAddr) {
        self.cpu.set_pc(pc.raw());
    }

    /// Fetch, decode and execute one instruction.
    pub fn step(&mut self) -> Result<()> {
        if let Some(exc) = self.pending_exception.take() {
            self.take_exception(exc);
        }

        let pc = self.cpu.pc();
        self.bus.set_current_pc(pc);
        let hw1 = self.fetch_halfword(pc)?;

        let (inst, size) = if is_32bit_prefix(hw1) {
            let hw2 = self.fetch_halfword(pc.wrapping_add(2))?;
            (decode::decode32(hw1, hw2), 4u32)
        } else {
            (decode::decode16(hw1), 2u32)
        };

        // `r[PC]` stays on the current instruction while it executes so that
        // PC-relative operands observe the architectural value.
        self.cpu.set_pc(pc);
        self.cpu.branched = false;
        self.exec(inst)?;
        if !self.cpu.branched {
            self.cpu.set_pc(pc.wrapping_add(size));
        }
        self.cycles += 1;
        Ok(())
    }

    /// Runs until the step budget is exhausted or execution stops.
    ///
    /// Returns the number of instructions retired in this call.
    pub fn run(&mut self, max_steps: u64) -> Result<u64> {
        let mut executed = 0;
        while executed < max_steps {
            match self.step() {
                Ok(()) => executed += 1,
                Err(CoreError::StepLimit(_)) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(executed)
    }

    fn fetch_halfword(&mut self, addr: u32) -> Result<u16> {
        let value = self.bus.read(PhysAddr::new(addr), AccessWidth::HalfWord)?;
        Ok(value as u16)
    }

    /// Raises an architecture exception, to be taken before the next step.
    pub fn raise_exception(&mut self, number: u16) {
        if self.pending_exception.is_none() {
            self.pending_exception = Some(number);
        }
    }
}

/// ARMv6-M reserves the top five bits `0b11101`/`0b11110`/`0b11111` for 32-bit
/// encodings. We only need to recognise the prefix; the second halfword
/// disambiguates.
#[inline]
pub const fn is_32bit_prefix(hw: u16) -> bool {
    (hw & 0xF800) == 0xE800 || (hw & 0xF000) == 0xF000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_32bit_prefixes() {
        assert!(is_32bit_prefix(0xF000));
        assert!(is_32bit_prefix(0xF3AF));
        assert!(is_32bit_prefix(0xE800));
        assert!(!is_32bit_prefix(0x2000));
        assert!(!is_32bit_prefix(0x4770));
    }
}
