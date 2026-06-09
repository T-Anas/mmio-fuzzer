use crate::access::{Access, AccessWidth};
use crate::addr::PhysAddr;
use crate::error::Result;

/// A data bus the CPU core can talk to.
///
/// Everything interesting about `mmio-fuzz` happens behind this trait: the
/// concrete implementation decides whether a word comes from RAM, from a
/// scripted peripheral model, or from the fuzzer's input tape.
pub trait Bus {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Result<u32>;
    fn write(&mut self, addr: PhysAddr, width: AccessWidth, value: u32) -> Result<()>;

    /// Convenience helpers used by the core and by tests.
    fn read_word(&mut self, addr: PhysAddr) -> Result<u32> {
        self.read(addr, AccessWidth::Word)
    }

    fn write_word(&mut self, addr: PhysAddr, value: u32) -> Result<()> {
        self.write(addr, AccessWidth::Word, value)
    }
}

/// A sink for bus transactions.
///
/// The tracer registers an observer with the bus; the bus calls it for every
/// access it services. Observers must stay cheap: they run inside the hot
/// execution loop.
pub trait AccessObserver {
    fn observe(&mut self, access: &Access);
}

/// Adapter that lets a closure act as an [`AccessObserver`].
///
/// A blanket `impl<F: FnMut(&Access)> AccessObserver for F` would overlap with
/// the concrete implementations below, so we use a tiny newtype instead.
pub struct FnObserver<F>(pub F);

impl<F: FnMut(&Access)> AccessObserver for FnObserver<F> {
    #[inline]
    fn observe(&mut self, access: &Access) {
        (self.0)(access)
    }
}

/// An observer that simply counts accesses, useful in tests and benchmarks.
#[derive(Default, Debug, Clone, Copy)]
pub struct AccessCounter {
    pub reads: u64,
    pub writes: u64,
    pub fetches: u64,
}

impl AccessCounter {
    pub fn total(&self) -> u64 {
        self.reads + self.writes + self.fetches
    }
}

impl AccessObserver for AccessCounter {
    fn observe(&mut self, access: &Access) {
        match access.kind {
            crate::access::AccessKind::Read => self.reads += 1,
            crate::access::AccessKind::Write => self.writes += 1,
            crate::access::AccessKind::Fetch => self.fetches += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AccessKind;

    #[test]
    fn closure_can_be_an_observer() {
        let mut captured = Vec::new();
        {
            let mut obs = FnObserver(|a: &Access| captured.push(a.addr.raw()));
            obs.observe(&Access::new(
                AccessKind::Read,
                PhysAddr::new(0x4000_0000),
                AccessWidth::Word,
                0,
                0,
                0,
            ));
        }
        assert_eq!(captured, vec![0x4000_0000]);
    }

    #[test]
    fn counter_counts_by_kind() {
        let mut c = AccessCounter::default();
        c.observe(&Access::new(
            AccessKind::Read,
            PhysAddr::ZERO,
            AccessWidth::Word,
            0,
            0,
            0,
        ));
        c.observe(&Access::new(
            AccessKind::Fetch,
            PhysAddr::ZERO,
            AccessWidth::HalfWord,
            0,
            0,
            1,
        ));
        assert_eq!((c.reads, c.fetches, c.total()), (1, 1, 2));
    }
}
