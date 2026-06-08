use core::fmt;

use serde::{Deserialize, Serialize};

use crate::addr::PhysAddr;

/// Direction of a single memory access.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessKind {
    /// The core reads a value from the bus.
    Read,
    /// The core writes a value to the bus.
    Write,
    /// The core fetches an instruction.
    Fetch,
}

impl AccessKind {
    #[inline]
    pub const fn is_write(self) -> bool {
        matches!(self, AccessKind::Write)
    }

    #[inline]
    pub const fn is_read(self) -> bool {
        matches!(self, AccessKind::Read | AccessKind::Fetch)
    }
}

impl fmt::Display for AccessKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AccessKind::Read => "read",
            AccessKind::Write => "write",
            AccessKind::Fetch => "fetch",
        };
        f.write_str(s)
    }
}

/// Transfer width of a memory access, in bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessWidth {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
}

impl AccessWidth {
    #[inline]
    pub const fn bytes(self) -> u32 {
        self as u32
    }

    /// Number of bits occupied by this width.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.bytes() * 8
    }

    /// Mask that keeps only the bits a transfer of this width can carry.
    #[inline]
    pub const fn mask(self) -> u32 {
        match self {
            AccessWidth::Byte => 0xff,
            AccessWidth::HalfWord => 0xffff,
            AccessWidth::Word => 0xffff_ffff,
        }
    }

    /// Truncates a 32-bit value to this transfer width.
    #[inline]
    pub const fn truncate(self, value: u32) -> u32 {
        value & self.mask()
    }
}

impl fmt::Display for AccessWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AccessWidth::Byte => "u8",
            AccessWidth::HalfWord => "u16",
            AccessWidth::Word => "u32",
        };
        f.write_str(s)
    }
}

/// A fully described bus transaction, as observed by the tracer.
///
/// `seq` is a monotonic counter owned by the backend. It gives the inference
/// layer a total order over accesses even when several peripherals are touched
/// within the same instruction.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Access {
    pub kind: AccessKind,
    pub addr: PhysAddr,
    pub width: AccessWidth,
    /// Written value for stores, or the value returned by the bus for loads.
    pub value: u32,
    /// Program counter that produced the access.
    pub pc: u32,
    /// Monotonic access index within a run.
    pub seq: u64,
}

impl Access {
    #[inline]
    pub const fn new(
        kind: AccessKind,
        addr: PhysAddr,
        width: AccessWidth,
        value: u32,
        pc: u32,
        seq: u64,
    ) -> Self {
        Self {
            kind,
            addr,
            width,
            value,
            pc,
            seq,
        }
    }

    /// Value truncated to the transfer width.
    #[inline]
    pub const fn masked_value(&self) -> u32 {
        self.width.truncate(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_expose_bytes_and_masks() {
        assert_eq!(AccessWidth::Byte.bytes(), 1);
        assert_eq!(AccessWidth::HalfWord.mask(), 0xffff);
        assert_eq!(AccessWidth::Word.mask(), 0xffff_ffff);
    }

    #[test]
    fn truncation_keeps_low_bits() {
        assert_eq!(AccessWidth::Byte.truncate(0x1234_abcd), 0xcd);
        assert_eq!(AccessWidth::HalfWord.truncate(0x1234_abcd), 0xabcd);
    }

    #[test]
    fn access_masks_the_value() {
        let a = Access::new(
            AccessKind::Write,
            PhysAddr::new(0x4000_0000),
            AccessWidth::Byte,
            0xdead_beef,
            0x0800_0000,
            7,
        );
        assert_eq!(a.masked_value(), 0xef);
        assert_eq!(a.seq, 7);
    }
}
