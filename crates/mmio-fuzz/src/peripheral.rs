//! The peripheral model used during emulation.
//!
//! This is where the inferred [`HardwareModel`] meets the `Bus`: reads to a
//! known register are answered from the register's list of plausible values,
//! biased by the fuzz input so mutations steer device interactions.

use std::collections::HashMap;

use mmio_core::{AccessWidth, PhysAddr};
use mmio_emu::memory::MmioHandler;
use mmio_infer::HardwareModel;

use crate::input::Input;

/// An MMIO handler driven by an inferred model and a fuzz input.
pub struct PeripheralModel {
    model: HardwareModel,
    input: Input,
    /// Forced read values, used during value discovery and for known-ready
    /// status registers.
    overrides: HashMap<u32, u32>,
    /// Per-address read counter, so repeated reads can return different
    /// plausible values.
    counters: HashMap<u32, u32>,
    /// Remaining reads that should return zero for read-to-clear registers.
    clear_budget: HashMap<u32, u32>,
    /// Value returned for registers with no model at all.
    pub fallback: u32,
    /// When false, the model is ignored and `fallback` is returned always.
    pub use_model: bool,
}

impl PeripheralModel {
    pub fn new(model: HardwareModel, input: Input) -> Self {
        Self {
            model,
            input,
            overrides: HashMap::new(),
            counters: HashMap::new(),
            clear_budget: HashMap::new(),
            fallback: 0,
            use_model: true,
        }
    }

    pub fn with_overrides(mut self, overrides: HashMap<u32, u32>) -> Self {
        self.overrides = overrides;
        self
    }

    pub fn with_fallback(mut self, fallback: u32) -> Self {
        self.fallback = fallback;
        self
    }

    /// Selects one value from `values` using the input and read counter.
    fn select(&self, addr: u32, counter: u32, values: &[u32]) -> u32 {
        if values.is_empty() {
            return self.fallback;
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(&addr.to_le_bytes());
        hasher.update(&counter.to_le_bytes());
        hasher.update(&self.input.bytes);
        let digest = hasher.finalize();
        let index =
            (u16::from_le_bytes([digest.as_bytes()[0], digest.as_bytes()[1]]) as usize) % values.len();
        values[index]
    }

    /// A value derived purely from the input, for unknown registers.
    fn hash_value(&self, addr: u32, counter: u32) -> u32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"unknown");
        hasher.update(&addr.to_le_bytes());
        hasher.update(&counter.to_le_bytes());
        hasher.update(&self.input.bytes);
        let digest = hasher.finalize();
        u32::from_le_bytes([
            digest.as_bytes()[0],
            digest.as_bytes()[1],
            digest.as_bytes()[2],
            digest.as_bytes()[3],
        ])
    }
}

impl MmioHandler for PeripheralModel {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Option<u32> {
        let a = addr.raw();

        let counter = {
            let entry = self.counters.entry(a).or_insert(0);
            let current = *entry;
            *entry = entry.wrapping_add(1);
            current
        };

        if let Some(value) = self.overrides.get(&a).copied() {
            return Some(value & width.mask());
        }

        let value = if !self.use_model {
            self.fallback
        } else if let Some(constant) = self.model.register(a).and_then(|reg| reg.constant_read) {
            // A register that always reads the same value is not a fuzzing
            // surface: return it deterministically.
            constant
        } else {
            match self.model.register(a) {
                Some(reg) => {
                    let values = reg.plausible_reads();
                    self.select(a, counter, &values)
                }
                None => self.hash_value(a, counter),
            }
        };

        // Read-to-clear: after one useful read, subsequent reads return zero
        // until the register is written again.
        if self.use_model {
            if let Some(reg) = self.model.register(a) {
                if reg.read_to_clear {
                    let budget = self.clear_budget.entry(a).or_insert(0);
                    if *budget > 0 {
                        *budget -= 1;
                        return Some(0);
                    }
                    *budget = 1;
                }
            }
        }

        Some(value & width.mask())
    }

    fn write(&mut self, addr: PhysAddr, _width: AccessWidth, _value: u32) -> Option<()> {
        // A write arms the next read-to-clear cycle again.
        self.clear_budget.remove(&addr.raw());
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mmio_infer::register::RegisterModel;

    fn model_with(reg: RegisterModel) -> HardwareModel {
        HardwareModel::from_registers(vec![reg])
    }

    #[test]
    fn constant_register_is_returned() {
        let mut reg = RegisterModel::new(0x4000_0000);
        reg.reads = 1;
        reg.constant_read = Some(0xabcd);
        reg.width = AccessWidth::Word;
        let mut handler = PeripheralModel::new(model_with(reg), Input::new());
        let value = handler.read(PhysAddr::new(0x4000_0000), AccessWidth::Word).unwrap();
        assert_eq!(value, 0xabcd);
    }

    #[test]
    fn overrides_take_priority() {
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new());
        let mut overrides = HashMap::new();
        overrides.insert(0x4000_0000u32, 0x55u32);
        handler = handler.with_overrides(overrides);
        assert_eq!(
            handler.read(PhysAddr::new(0x4000_0000), AccessWidth::Word).unwrap(),
            0x55
        );
    }

    #[test]
    fn read_to_clear_returns_zero_after_first_read() {
        let mut reg = RegisterModel::new(0x4000_0000);
        reg.reads = 2;
        reg.read_to_clear = true;
        reg.constant_read = Some(0x1);
        reg.ready_value = Some(0x1);
        reg.width = AccessWidth::Word;
        let mut handler = PeripheralModel::new(model_with(reg), Input::new());
        let first = handler.read(PhysAddr::new(0x4000_0000), AccessWidth::Word).unwrap();
        let second = handler.read(PhysAddr::new(0x4000_0000), AccessWidth::Word).unwrap();
        assert_eq!(first, 0x1);
        assert_eq!(second, 0);
    }

    #[test]
    fn width_masks_the_value() {
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new());
        handler = handler.with_fallback(0xdead_beef);
        handler.use_model = false;
        assert_eq!(
            handler.read(PhysAddr::new(0x4000_0000), AccessWidth::Byte).unwrap(),
            0xef
        );
    }
}
