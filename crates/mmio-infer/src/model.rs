//! The assembled hardware model: peripherals, registers and serialisation.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::register::RegisterModel;

/// Size of the address block a peripheral is grouped into.
///
/// 4 KiB matches the granularity of most Cortex-M peripheral framings without
/// being so fine that each register becomes its own "peripheral".
pub const BLOCK_SIZE: u32 = 0x1000;

/// A group of registers sharing an aligned address block.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeripheralBlock {
    pub name: String,
    pub base: u32,
    pub size: u32,
    pub registers: Vec<RegisterModel>,
}

impl PeripheralBlock {
    pub fn new(base: u32) -> Self {
        Self {
            name: format!("mmio@{base:#010x}"),
            base,
            size: BLOCK_SIZE,
            registers: Vec::new(),
        }
    }

    pub fn register(&self, addr: u32) -> Option<&RegisterModel> {
        self.registers.iter().find(|r| r.addr == addr)
    }
}

/// The whole inferred device tree.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HardwareModel {
    pub blocks: Vec<PeripheralBlock>,
}

impl HardwareModel {
    pub const fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// The aligned block index for an address.
    #[inline]
    pub const fn block_base(addr: u32) -> u32 {
        addr & !(BLOCK_SIZE - 1)
    }

    /// Inserts or returns the block that owns `addr`.
    pub fn block_for(&mut self, addr: u32) -> &mut PeripheralBlock {
        let base = Self::block_base(addr);
        if let Some(index) = self.blocks.iter().position(|b| b.base == base) {
            return &mut self.blocks[index];
        }
        self.blocks.push(PeripheralBlock::new(base));
        self.blocks.last_mut().expect("just pushed")
    }

    pub fn register(&self, addr: u32) -> Option<&RegisterModel> {
        self.blocks
            .iter()
            .find(|b| b.base == Self::block_base(addr))
            .and_then(|b| b.register(addr))
    }

    /// Suggested value for a peripheral read, or `fallback` when unknown.
    pub fn suggest_read(&self, addr: u32, fallback: u32) -> u32 {
        match self.register(addr) {
            Some(reg) => {
                let values = reg.plausible_reads();
                values.first().copied().unwrap_or(fallback)
            }
            None => fallback,
        }
    }

    pub fn register_count(&self) -> usize {
        self.blocks.iter().map(|b| b.registers.len()).sum()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("model is serialisable")
    }

    pub fn from_json(text: &str) -> serde_json::Result<Self> {
        serde_json::from_str(text)
    }

    /// Builds a model from a flat list of registers, grouping into blocks.
    pub fn from_registers(registers: Vec<RegisterModel>) -> Self {
        let mut sorted = registers;
        sorted.sort_by_key(|r| r.addr);
        let mut model = Self::new();
        for reg in sorted {
            model.block_for(reg.addr).registers.push(reg);
        }
        model
    }

    /// One-line-per-register report.
    pub fn report(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            out.push_str(&format!("{}\n", block.name));
            let mut regs: BTreeMap<u32, &RegisterModel> = BTreeMap::new();
            for r in &block.registers {
                regs.insert(r.addr, r);
            }
            for (_, reg) in regs {
                out.push_str(&format!("  {}\n", reg.summary()));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_registers_into_blocks() {
        let mut a = RegisterModel::new(0x4000_0000);
        a.reads = 1;
        let mut b = RegisterModel::new(0x4000_0004);
        b.writes = 1;
        let mut c = RegisterModel::new(0x4000_1000);
        c.reads = 1;

        let model = HardwareModel::from_registers(vec![a, b, c]);
        assert_eq!(model.blocks.len(), 2);
        assert_eq!(model.register_count(), 3);
        assert!(model.register(0x4000_0004).is_some());
    }

    #[test]
    fn json_roundtrip() {
        let mut reg = RegisterModel::new(0x4000_0000);
        reg.constant_read = Some(0x1234);
        let model = HardwareModel::from_registers(vec![reg]);
        let json = model.to_json_pretty();
        let back = HardwareModel::from_json(&json).unwrap();
        assert_eq!(back.register_count(), 1);
        assert_eq!(
            back.register(0x4000_0000).unwrap().constant_read,
            Some(0x1234)
        );
    }

    #[test]
    fn block_base_aligns_down() {
        assert_eq!(HardwareModel::block_base(0x4000_1abc), 0x4000_1000);
        assert_eq!(HardwareModel::block_base(0x4000_1fff), 0x4000_1000);
        assert_eq!(HardwareModel::block_base(0x4000_2000), 0x4000_2000);
    }
}
