use core::fmt;
use core::ops::{Add, AddAssign, Sub};

use serde::{Deserialize, Serialize};

/// A physical address inside the emulated target's address space.
///
/// Cortex-M targets address at most 4 GiB, so a `u32` is the natural backing
/// type and keeps [`Access`](crate::Access) small enough to copy around freely.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct PhysAddr(u32);

impl PhysAddr {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// The address as a plain integer.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns a new address shifted by `offset`, wrapping at the 32-bit
    /// boundary. Hardware wraps too, and fuzzing *should* exercise that.
    #[inline]
    pub const fn wrapping_offset(self, offset: u32) -> Self {
        Self(self.0.wrapping_add(offset))
    }

    #[inline]
    pub const fn is_aligned_to(self, alignment: u32) -> bool {
        alignment != 0 && self.0 % alignment == 0
    }

    /// Offset of this address relative to `base`, or `None` when below it.
    #[inline]
    pub const fn offset_from(self, base: Self) -> Option<u32> {
        self.0.checked_sub(base.0)
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl From<u32> for PhysAddr {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<PhysAddr> for u32 {
    #[inline]
    fn from(value: PhysAddr) -> Self {
        value.0
    }
}

impl Add<u32> for PhysAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}

impl AddAssign<u32> for PhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = u32;

    #[inline]
    fn sub(self, rhs: PhysAddr) -> Self::Output {
        self.0.wrapping_sub(rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_from_is_checked() {
        let base = PhysAddr::new(0x4000_0000);
        assert_eq!(PhysAddr::new(0x4000_0010).offset_from(base), Some(0x10));
        assert_eq!(PhysAddr::new(0x3fff_ffff).offset_from(base), None);
    }

    #[test]
    fn addition_wraps_like_hardware() {
        let a = PhysAddr::new(u32::MAX);
        assert_eq!(a + 1, PhysAddr::ZERO);
    }

    #[test]
    fn alignment_boundaries() {
        assert!(PhysAddr::new(0x1000).is_aligned_to(4));
        assert!(!PhysAddr::new(0x1002).is_aligned_to(4));
        assert!(!PhysAddr::new(0).is_aligned_to(0));
    }
}
