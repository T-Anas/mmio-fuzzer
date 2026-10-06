//! Shadow memory: precise out-of-bounds detection inside mapped RAM.
//!
//! Unmapped guard holes catch accesses that leave a region entirely. They miss
//! the common case of an off-by-a-few-bytes access that stays inside RAM. The
//! sanitizer tracks *redzones* — poisoned byte ranges around buffers, stack
//! frames and heap objects — and flags any access that overlaps one.
//!
//! The model is deliberately coarse (interval list, not a byte shadow map):
//! a target has a handful of interesting buffers, so an interval check is both
//! fast enough and much simpler than a full shadow map.

use mmio_core::{AccessKind, AccessWidth, CoreError, PhysAddr};

/// Classification of an address for the sanitizer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShadowKind {
    /// Ordinary memory; accesses are allowed.
    Valid,
    /// Poisoned memory; any access is an out-of-bounds violation.
    Redzone,
}

/// A contiguous range with a sanitizer classification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShadowRegion {
    pub base: u32,
    pub size: u32,
    pub kind: ShadowKind,
}

impl ShadowRegion {
    /// Whether `[start, end)` overlaps this region.
    fn overlaps(&self, start: u64, end: u64) -> bool {
        let region_start = self.base as u64;
        let region_end = region_start + self.size as u64;
        start < region_end && region_start < end
    }
}

/// A set of shadow regions consulted on every access.
#[derive(Clone, Default, Debug)]
pub struct Shadow {
    regions: Vec<ShadowRegion>,
}

impl Shadow {
    pub const fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Marks `[base, base+size)` as an allowed buffer.
    pub fn allow(&mut self, base: u32, size: u32) {
        self.push(base, size, ShadowKind::Valid);
    }

    /// Marks `[base, base+size)` as a redzone. Any access is a violation.
    pub fn poison(&mut self, base: u32, size: u32) {
        self.push(base, size, ShadowKind::Redzone);
    }

    fn push(&mut self, base: u32, size: u32, kind: ShadowKind) {
        if size == 0 {
            return;
        }
        self.regions.push(ShadowRegion { base, size, kind });
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.regions.len()
    }

    pub fn regions(&self) -> &[ShadowRegion] {
        &self.regions
    }

    /// True when the byte at `addr` is poisoned.
    pub fn is_poisoned(&self, addr: u32) -> bool {
        let point = addr as u64;
        self.regions
            .iter()
            .any(|r| r.kind == ShadowKind::Redzone && r.overlaps(point, point + 1))
    }

    /// Checks a transfer, returning an error if any byte lands in a redzone.
    pub fn check(
        &self,
        addr: PhysAddr,
        kind: AccessKind,
        width: AccessWidth,
    ) -> Result<(), CoreError> {
        if self.is_empty() {
            return Ok(());
        }
        let start = addr.raw() as u64;
        let end = start + width.bytes() as u64;
        if self
            .regions
            .iter()
            .any(|r| r.kind == ShadowKind::Redzone && r.overlaps(start, end))
        {
            return Err(CoreError::OutOfBounds { addr, kind, width });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_shadow_allows_everything() {
        let shadow = Shadow::new();
        assert!(shadow
            .check(
                PhysAddr::new(0x2000_0000),
                AccessKind::Read,
                AccessWidth::Word
            )
            .is_ok());
    }

    #[test]
    fn access_into_redzone_is_out_of_bounds() {
        let mut shadow = Shadow::new();
        shadow.allow(0x2000_0000, 0x100);
        shadow.poison(0x2000_0100, 0x10);
        assert!(shadow
            .check(
                PhysAddr::new(0x2000_0000),
                AccessKind::Write,
                AccessWidth::Word
            )
            .is_ok());
        assert!(matches!(
            shadow.check(
                PhysAddr::new(0x2000_00ff),
                AccessKind::Write,
                AccessWidth::HalfWord
            ),
            Err(CoreError::OutOfBounds { .. })
        ));
        assert!(matches!(
            shadow.check(
                PhysAddr::new(0x2000_0104),
                AccessKind::Read,
                AccessWidth::Byte
            ),
            Err(CoreError::OutOfBounds { .. })
        ));
        assert!(shadow
            .check(
                PhysAddr::new(0x2000_0200),
                AccessKind::Read,
                AccessWidth::Byte
            )
            .is_ok());
    }

    #[test]
    fn is_poisoned_point_query() {
        let mut shadow = Shadow::new();
        shadow.poison(0x2000_0300, 4);
        assert!(shadow.is_poisoned(0x2000_0300));
        assert!(shadow.is_poisoned(0x2000_0303));
        assert!(!shadow.is_poisoned(0x2000_0304));
    }
}
