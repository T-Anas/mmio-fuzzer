//! Per-register model and the queries the fuzzer asks of it.

use std::collections::BTreeSet;

use mmio_core::AccessWidth;
use serde::{Deserialize, Serialize};

/// How a register is accessed by the firmware.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessStyle {
    /// Only ever read (status registers, ID registers).
    ReadOnly,
    /// Only ever written (command, configuration).
    WriteOnly,
    /// Both read and written (control/status pairs, counters).
    ReadWrite,
    /// Never observed in a usable way.
    Unknown,
}

/// What we have learned about one MMIO address.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterModel {
    /// Absolute address.
    pub addr: u32,
    /// Widest transfer observed at this address.
    pub width: AccessWidth,
    pub reads: u64,
    pub writes: u64,
    /// All distinct values returned by reads.
    pub read_values: BTreeSet<u32>,
    /// All distinct values written.
    pub write_values: BTreeSet<u32>,
    /// Set when every read returned the same value.
    pub constant_read: Option<u32>,
    /// True when the firmware spins reading this register (a status bit).
    pub polled: bool,
    /// Program counters from which the register was polled.
    pub poll_sites: BTreeSet<u32>,
    /// The value that terminated a poll loop, if any.
    pub ready_value: Option<u32>,
    /// Bits that changed between the first and last read of a poll loop.
    pub ready_mask: u32,
    /// Reading seems to clear bits (no intervening write).
    pub read_to_clear: bool,
    /// A write with set bits is followed by reads with fewer bits set.
    pub write_to_clear_hint: bool,
}

impl RegisterModel {
    pub fn new(addr: u32) -> Self {
        Self {
            addr,
            width: AccessWidth::Byte,
            reads: 0,
            writes: 0,
            read_values: BTreeSet::new(),
            write_values: BTreeSet::new(),
            constant_read: None,
            polled: false,
            poll_sites: BTreeSet::new(),
            ready_value: None,
            ready_mask: 0,
            read_to_clear: false,
            write_to_clear_hint: false,
        }
    }

    pub fn access_style(&self) -> AccessStyle {
        match (self.reads > 0, self.writes > 0) {
            (true, false) => AccessStyle::ReadOnly,
            (false, true) => AccessStyle::WriteOnly,
            (true, true) => AccessStyle::ReadWrite,
            (false, false) => AccessStyle::Unknown,
        }
    }

    /// Widens the modelled transfer width to cover `width`.
    pub fn widen_to(&mut self, width: AccessWidth) {
        let rank = |w: AccessWidth| w.bytes();
        if rank(width) > rank(self.width) {
            self.width = width;
        }
    }

    /// A small, ordered set of read values worth trying during fuzzing.
    ///
    /// The order matters: the constant value first (keeps already-working
    /// paths alive), then the ready value (unblocks polling loops), then
    /// explorations of single bits.
    pub fn plausible_reads(&self) -> Vec<u32> {
        let mask = self.width.mask();
        let mut out = Vec::new();
        let push = |v: u32, out: &mut Vec<u32>| {
            let v = v & mask;
            if !out.contains(&v) {
                out.push(v);
            }
        };

        if let Some(c) = self.constant_read {
            push(c, &mut out);
        }
        if let Some(r) = self.ready_value {
            push(r, &mut out);
        }
        for v in &self.read_values {
            push(*v, &mut out);
        }
        // Bit-level explorations: one bit set at a time, then all ones and zero.
        let mut bit = 1u32;
        while bit != 0 && bit <= mask {
            push(bit, &mut out);
            bit <<= 1;
        }
        push(mask, &mut out);
        push(0, &mut out);
        out
    }

    /// Number of distinct read values ever observed.
    pub fn read_entropy(&self) -> usize {
        self.read_values.len()
    }

    /// Short human-readable summary, used by `mmio-fuzz infer`.
    pub fn summary(&self) -> String {
        let style = match self.access_style() {
            AccessStyle::ReadOnly => "RO",
            AccessStyle::WriteOnly => "WO",
            AccessStyle::ReadWrite => "RW",
            AccessStyle::Unknown => "??",
        };
        let mut notes = Vec::new();
        if let Some(c) = self.constant_read {
            notes.push(format!("const={c:#010x}"));
        }
        if self.polled {
            notes.push("polled".to_string());
            if let Some(r) = self.ready_value {
                notes.push(format!("ready={r:#010x}"));
            }
        }
        if self.read_to_clear {
            notes.push("r2c".to_string());
        }
        if self.write_to_clear_hint {
            notes.push("w1c?".to_string());
        }
        let suffix = if notes.is_empty() {
            String::new()
        } else {
            format!("  [{}]", notes.join(" "))
        };
        format!(
            "{:#010x}  {style}  {} r/{} w  width={}{suffix}",
            self.addr,
            self.reads,
            self.writes,
            self.width
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_reflects_traffic() {
        let mut r = RegisterModel::new(0x4000_0000);
        assert_eq!(r.access_style(), AccessStyle::Unknown);
        r.reads = 3;
        assert_eq!(r.access_style(), AccessStyle::ReadOnly);
        r.writes = 1;
        assert_eq!(r.access_style(), AccessStyle::ReadWrite);
    }

    #[test]
    fn widening_keeps_the_largest() {
        let mut r = RegisterModel::new(0x4000_0000);
        r.widen_to(AccessWidth::HalfWord);
        r.widen_to(AccessWidth::Byte);
        assert_eq!(r.width, AccessWidth::HalfWord);
        r.widen_to(AccessWidth::Word);
        assert_eq!(r.width, AccessWidth::Word);
    }

    #[test]
    fn plausible_reads_prioritise_known_values() {
        let mut r = RegisterModel::new(0x4000_0000);
        r.width = AccessWidth::Byte;
        r.constant_read = Some(0x42);
        r.ready_value = Some(0x01);
        let values = r.plausible_reads();
        assert_eq!(values[0], 0x42);
        assert_eq!(values[1], 0x01);
        assert!(values.contains(&0xff));
        assert!(values.contains(&0));
    }
}
