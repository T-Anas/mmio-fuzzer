//! The observed MMIO trace.
//!
//! An [`AccessLog`] is just a growable vector of bus transactions. It is also
//! an [`AccessObserver`], so it can be attached directly to a
//! [`FlatMemory`](mmio_emu::FlatMemory) during a run and then handed to the
//! inference pass.

use mmio_core::{Access, AccessKind, AccessObserver};

/// Start of the ARMv6-M peripheral window.
pub const PERIPHERAL_START: u32 = 0x4000_0000;
/// End (exclusive) of the ARMv6-M peripheral window.
pub const PERIPHERAL_END: u32 = 0x6000_0000;
/// Start of the private peripheral bus (NVIC, SysTick, SCB).
pub const SYSTEM_START: u32 = 0xE000_0000;
/// End (exclusive) of the private peripheral bus.
pub const SYSTEM_END: u32 = 0xE010_0000;

/// Heuristic: does `addr` look like a memory-mapped peripheral?
///
/// Real firmware may place peripherals outside these windows, but the ARMv6-M
/// architecture defines exactly these regions, and the fuzzer can be told to
/// treat additional ranges as MMIO if a vendor strays.
#[inline]
pub const fn is_mmio_address(addr: u32) -> bool {
    (addr >= PERIPHERAL_START && addr < PERIPHERAL_END)
        || (addr >= SYSTEM_START && addr < SYSTEM_END)
}

/// An append-only journal of bus accesses.
#[derive(Clone, Default, Debug)]
pub struct AccessLog {
    accesses: Vec<Access>,
}

impl AccessLog {
    pub const fn new() -> Self {
        Self {
            accesses: Vec::new(),
        }
    }

    pub fn from_vec(accesses: Vec<Access>) -> Self {
        Self { accesses }
    }

    pub fn push(&mut self, access: Access) {
        self.accesses.push(access);
    }

    pub fn len(&self) -> usize {
        self.accesses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.accesses.is_empty()
    }

    pub fn clear(&mut self) {
        self.accesses.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Access> {
        self.accesses.iter()
    }

    pub fn as_slice(&self) -> &[Access] {
        &self.accesses
    }

    /// Iterator over accesses that target memory-mapped peripherals.
    pub fn mmio(&self) -> impl Iterator<Item = &Access> {
        self.accesses
            .iter()
            .filter(|a| is_mmio_address(a.addr.raw()))
    }

    /// Number of transactions in the peripheral window.
    pub fn mmio_len(&self) -> usize {
        self.mmio().count()
    }

    /// Distinct peripheral addresses touched, sorted.
    pub fn touched_addresses(&self) -> Vec<u32> {
        let mut addrs: Vec<u32> = self.mmio().map(|a| a.addr.raw()).collect();
        addrs.sort_unstable();
        addrs.dedup();
        addrs
    }

    /// Counts of reads and writes in the peripheral window.
    pub fn mmio_reads_writes(&self) -> (u64, u64) {
        let mut reads = 0;
        let mut writes = 0;
        for a in self.mmio() {
            match a.kind {
                AccessKind::Write => writes += 1,
                _ => reads += 1,
            }
        }
        (reads, writes)
    }
}

impl AccessObserver for AccessLog {
    #[inline]
    fn observe(&mut self, access: &Access) {
        self.accesses.push(*access);
    }
}

impl FromIterator<Access> for AccessLog {
    fn from_iter<T: IntoIterator<Item = Access>>(iter: T) -> Self {
        Self {
            accesses: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mmio_core::{AccessWidth, PhysAddr};

    fn access(addr: u32, kind: AccessKind) -> Access {
        Access::new(kind, PhysAddr::new(addr), AccessWidth::Word, 0, 0, 0)
    }

    #[test]
    fn address_classification() {
        assert!(is_mmio_address(0x4000_0000));
        assert!(is_mmio_address(0x5fff_ffff));
        assert!(!is_mmio_address(0x6000_0000));
        assert!(is_mmio_address(0xe000_e010));
        assert!(!is_mmio_address(0x2000_0000));
    }

    #[test]
    fn filters_peripheral_traffic() {
        let mut log = AccessLog::new();
        log.push(access(0x2000_0000, AccessKind::Write));
        log.push(access(0x4000_0000, AccessKind::Write));
        log.push(access(0x4000_0004, AccessKind::Read));
        log.push(access(0x4000_0000, AccessKind::Read));
        assert_eq!(log.mmio_len(), 3);
        assert_eq!(log.touched_addresses(), vec![0x4000_0000, 0x4000_0004]);
        assert_eq!(log.mmio_reads_writes(), (2, 1));
    }

    #[test]
    fn acts_as_an_observer() {
        let mut log = AccessLog::new();
        log.observe(&access(0x4000_0008, AccessKind::Read));
        assert_eq!(log.len(), 1);
    }
}
