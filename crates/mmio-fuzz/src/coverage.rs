//! AFL-style edge coverage.
//!
//! We record the transition between consecutive program counters into a
//! fixed-size bitmap. That is enough to tell whether a mutated peripheral
//! response sent the firmware somewhere new, which is all the engine needs.

use serde::{Deserialize, Serialize};

/// Number of buckets in the coverage map.
pub const MAP_SIZE: usize = 1 << 16;

/// A coverage bitmap with a notion of edges already seen.
#[derive(Clone, Serialize, Deserialize)]
pub struct Coverage {
    #[serde(skip)]
    buckets: Vec<u8>,
    #[serde(skip)]
    prev: u32,
    edges: usize,
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
    }
}

impl Coverage {
    pub fn new() -> Self {
        Self {
            buckets: vec![0; MAP_SIZE],
            prev: 0,
            edges: 0,
        }
    }

    /// Clears the map for a new run.
    pub fn reset(&mut self) {
        self.buckets.iter_mut().for_each(|b| *b = 0);
        self.prev = 0;
        self.edges = 0;
    }

    /// Records execution of `pc`, hashing in the previous location to form an
    /// edge.
    #[inline]
    pub fn record(&mut self, pc: u32) {
        let index = (((pc >> 1) ^ self.prev) as usize) & (MAP_SIZE - 1);
        if self.buckets[index] == 0 {
            self.edges += 1;
        }
        self.buckets[index] = self.buckets[index].saturating_add(1);
        self.prev = pc >> 1;
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn bitmap(&self) -> &[u8] {
        &self.buckets
    }

    /// Number of edges present in `other` but not yet in `self`.
    pub fn count_new(&self, other: &Coverage) -> usize {
        self.buckets
            .iter()
            .zip(&other.buckets)
            .filter(|(mine, theirs)| **mine == 0 && **theirs > 0)
            .count()
    }

    /// Merges `other` into `self`, returning how many edges were new.
    pub fn merge(&mut self, other: &Coverage) -> usize {
        let mut added = 0;
        for (mine, theirs) in self.buckets.iter_mut().zip(&other.buckets) {
            if *mine == 0 && *theirs > 0 {
                added += 1;
            }
            *mine |= *theirs;
        }
        self.edges += added;
        added
    }

    /// Stable digest used to deduplicate corpus entries.
    pub fn digest(&self) -> [u8; 32] {
        *blake3::hash(&self.buckets).as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_creates_edges() {
        let mut cov = Coverage::new();
        cov.record(0x100);
        cov.record(0x104);
        cov.record(0x108);
        assert_eq!(cov.edges(), 3);
        // Re-running the same sequence is deterministic and adds nothing.
        let edges_after_first_pass = cov.edges();
        cov.reset();
        cov.record(0x100);
        cov.record(0x104);
        cov.record(0x108);
        assert_eq!(cov.edges(), edges_after_first_pass);
    }

    #[test]
    fn merge_reports_new_edges() {
        let mut a = Coverage::new();
        a.record(0x10);
        a.record(0x20);

        let mut b = Coverage::new();
        b.record(0x10);
        b.record(0x30);

        let added = a.merge(&b);
        assert_eq!(added, 1);
        assert_eq!(a.edges(), 3);
    }

    #[test]
    fn count_new_is_directional() {
        let mut a = Coverage::new();
        a.record(0x1);

        let mut b = Coverage::new();
        b.record(0x1);
        b.record(0x2);
        b.record(0x4);
        b.record(0x8);

        // `b` has edges `a` has never seen.
        assert!(a.count_new(&b) >= 1);
        // And `a` has nothing `b` is missing.
        assert_eq!(b.count_new(&a), 0);
    }
}
