use serde::{Deserialize, Serialize};

use crate::addr::PhysAddr;

/// Coarse classification of a memory region.
///
/// The fuzzer only cares about a handful of behaviours: whether a region is
/// writable state, immutable code, or an MMIO window whose reads must be
/// modelled. Keeping the taxonomy small avoids over-fitting to any vendor.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionKind {
    /// Non-volatile code, usually aliased at 0x0000_0000 on reset.
    Flash,
    /// Volatile data. Reads and writes behave like ordinary RAM.
    Ram,
    /// Memory-mapped peripheral registers. The interesting part.
    Peripheral,
    /// Private peripheral bus: NVIC, SysTick, SCB.
    System,
    /// Anything else: external memory, vendor test windows...
    External,
}

impl RegionKind {
    #[inline]
    pub const fn is_mmio(self) -> bool {
        matches!(self, RegionKind::Peripheral | RegionKind::System)
    }
}

/// Access rights attached to a region.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permissions {
    pub const NONE: Self = Self {
        read: false,
        write: false,
        execute: false,
    };
    pub const RW: Self = Self {
        read: true,
        write: true,
        execute: false,
    };
    pub const RX: Self = Self {
        read: true,
        write: false,
        execute: true,
    };
    pub const RWX: Self = Self {
        read: true,
        write: true,
        execute: true,
    };

    #[inline]
    pub const fn with_write(self) -> Self {
        Self {
            write: true,
            ..self
        }
    }
}

/// A named, contiguous range of the physical address space.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub name: String,
    pub base: PhysAddr,
    pub size: u32,
    pub kind: RegionKind,
    pub perms: Permissions,
}

impl MemoryRegion {
    pub fn new(
        name: impl Into<String>,
        base: PhysAddr,
        size: u32,
        kind: RegionKind,
        perms: Permissions,
    ) -> Self {
        Self {
            name: name.into(),
            base,
            size,
            kind,
            perms,
        }
    }

    /// Exclusive end address, computed in 64 bits so full 4 GiB regions work.
    #[inline]
    pub const fn end(&self) -> u64 {
        self.base.raw() as u64 + self.size as u64
    }

    #[inline]
    pub const fn contains(&self, addr: PhysAddr) -> bool {
        let a = addr.raw() as u64;
        a >= self.base.raw() as u64 && a < self.end()
    }

    /// Offset of `addr` inside the region, or `None` when outside.
    #[inline]
    pub const fn offset(&self, addr: PhysAddr) -> Option<u32> {
        if self.contains(addr) {
            Some(addr.raw() - self.base.raw())
        } else {
            None
        }
    }
}

/// Ordered collection of regions describing a target's memory layout.
///
/// Lookups are linear; maps in practice contain a dozen entries, so a
/// radix tree would only add bugs.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct MemoryMap {
    regions: Vec<MemoryRegion>,
}

impl MemoryMap {
    pub const fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Inserts a region. Overlaps are rejected because a target with
    /// overlapping regions has an ambiguous layout we would rather flag.
    pub fn insert(&mut self, region: MemoryRegion) -> Result<(), MapError> {
        for existing in &self.regions {
            let disjoint = existing.end() <= region.base.raw() as u64
                || region.end() <= existing.base.raw() as u64;
            if !disjoint {
                return Err(MapError::Overlap {
                    existing: existing.name.clone(),
                    incoming: region.name,
                });
            }
        }
        self.regions.push(region);
        self.regions.sort_by_key(|r| r.base);
        Ok(())
    }

    pub fn find(&self, addr: PhysAddr) -> Option<&MemoryRegion> {
        self.regions.iter().find(|r| r.contains(addr))
    }

    pub fn region_of(&self, addr: PhysAddr) -> Option<&str> {
        self.find(addr).map(|r| r.name.as_str())
    }

    pub fn is_mmio(&self, addr: PhysAddr) -> bool {
        self.find(addr).is_some_and(|r| r.kind.is_mmio())
    }

    pub fn iter(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions.iter()
    }

    pub fn len(&self) -> usize {
        self.regions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MapError {
    #[error("region {incoming:?} overlaps existing region {existing:?}")]
    Overlap { existing: String, incoming: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(name: &str, base: u32, size: u32, kind: RegionKind) -> MemoryRegion {
        MemoryRegion::new(name, PhysAddr::new(base), size, kind, Permissions::RW)
    }

    #[test]
    fn contains_and_offset() {
        let r = region("flash", 0x0800_0000, 0x1_0000, RegionKind::Flash);
        assert!(r.contains(PhysAddr::new(0x0800_0000)));
        assert!(r.contains(PhysAddr::new(0x0800_ffff)));
        assert!(!r.contains(PhysAddr::new(0x0801_0000)));
        assert_eq!(r.offset(PhysAddr::new(0x0800_0004)), Some(4));
        assert_eq!(r.offset(PhysAddr::new(0x0801_0000)), None);
    }

    #[test]
    fn full_4gib_region_does_not_overflow() {
        let r = region("all", 0, u32::MAX, RegionKind::External);
        assert!(r.contains(PhysAddr::new(0xffff_fffe)));
    }

    #[test]
    fn map_rejects_overlaps_but_accepts_touching() {
        let mut map = MemoryMap::new();
        map.insert(region("a", 0x4000_0000, 0x1000, RegionKind::Peripheral))
            .unwrap();
        assert!(map
            .insert(region("b", 0x4000_0800, 0x1000, RegionKind::Peripheral))
            .is_err());
        map.insert(region("c", 0x4000_1000, 0x1000, RegionKind::Peripheral))
            .unwrap();
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn mmio_detection_matches_kind() {
        let mut map = MemoryMap::new();
        map.insert(region("sram", 0x2000_0000, 0x4000, RegionKind::Ram))
            .unwrap();
        map.insert(region("uart0", 0x4000_0000, 0x1000, RegionKind::Peripheral))
            .unwrap();
        assert!(!map.is_mmio(PhysAddr::new(0x2000_0000)));
        assert!(map.is_mmio(PhysAddr::new(0x4000_0010)));
        assert_eq!(map.region_of(PhysAddr::new(0x4000_0010)), Some("uart0"));
    }
}
