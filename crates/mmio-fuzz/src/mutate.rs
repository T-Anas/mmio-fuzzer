//! Input mutation.
//!
//! A small havoc mutator: no grammar, no structure, just the classic battery
//! of byte-level operations. It is seeded so runs are reproducible, which
//! matters more here than raw throughput because a finding is only useful if
//! you can replay it.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::input::Input;

/// Values that tend to be interesting on a memory-mapped bus.
pub const INTERESTING_WORDS: [u32; 14] = [
    0x0000_0000,
    0x0000_0001,
    0x0000_00ff,
    0x0000_ffff,
    0x0001_0000,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_ffff,
    0xdead_beef,
    0x5555_5555,
    0xaaaa_aaaa,
    0x4000_0000,
    0xe000_e000,
    0x0000_8000,
];

/// A reproducible havoc mutator.
pub struct Mutator {
    rng: ChaCha8Rng,
}

impl Mutator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// A short, mostly-zero input. The first execution of a run often does
    /// best with an empty input, so callers should seed with `Input::new()`
    /// and use this only for subsequent entries.
    pub fn random_input(&mut self, max_len: usize) -> Input {
        let len = self.rng.gen_range(0..=max_len);
        let bytes = (0..len).map(|_| self.rng.gen::<u8>()).collect();
        Input::from_vec(bytes)
    }

    /// Applies one to several havoc mutations in place.
    pub fn mutate(&mut self, input: &mut Input, max_len: usize) {
        if input.bytes.is_empty() {
            let byte = self.rng.gen::<u8>();
            input.bytes.push(byte);
        }
        let rounds = self.rng.gen_range(1..=4);
        for _ in 0..rounds {
            match self.rng.gen_range(0..7) {
                0 => self.flip_bit(input),
                1 => self.set_byte(input),
                2 => self.add_byte_value(input),
                3 => self.insert_byte(input, max_len),
                4 => self.delete_byte(input),
                5 => self.duplicate_chunk(input, max_len),
                _ => self.write_interesting_word(input),
            }
        }
    }

    fn flip_bit(&mut self, input: &mut Input) {
        if input.bytes.is_empty() {
            return;
        }
        let index = self.rng.gen_range(0..input.bytes.len());
        let bit = self.rng.gen_range(0..8);
        input.bytes[index] ^= 1 << bit;
    }

    fn set_byte(&mut self, input: &mut Input) {
        if input.bytes.is_empty() {
            return;
        }
        let index = self.rng.gen_range(0..input.bytes.len());
        input.bytes[index] = self.rng.gen::<u8>();
    }

    fn add_byte_value(&mut self, input: &mut Input) {
        if input.bytes.is_empty() {
            return;
        }
        let index = self.rng.gen_range(0..input.bytes.len());
        let delta = self.rng.gen_range(0..16i16) - 8;
        input.bytes[index] = (input.bytes[index] as i16 + delta) as u8;
    }

    fn insert_byte(&mut self, input: &mut Input, max_len: usize) {
        if input.bytes.len() >= max_len {
            return;
        }
        let index = self.rng.gen_range(0..=input.bytes.len());
        let byte = self.rng.gen::<u8>();
        input.bytes.insert(index, byte);
    }

    fn delete_byte(&mut self, input: &mut Input) {
        if input.bytes.len() <= 1 {
            return;
        }
        let index = self.rng.gen_range(0..input.bytes.len());
        input.bytes.remove(index);
    }

    fn duplicate_chunk(&mut self, input: &mut Input, max_len: usize) {
        if input.bytes.is_empty() || input.bytes.len() >= max_len {
            return;
        }
        let start = self.rng.gen_range(0..input.bytes.len());
        let len = self.rng.gen_range(1..=(input.bytes.len() - start));
        let chunk: Vec<u8> = input.bytes[start..start + len].to_vec();
        let at = self.rng.gen_range(0..=input.bytes.len());
        input.bytes.splice(at..at, chunk);
        input.bytes.truncate(max_len);
    }

    fn write_interesting_word(&mut self, input: &mut Input) {
        if input.bytes.len() < 4 {
            return;
        }
        let index = self.rng.gen_range(0..=input.bytes.len() - 4);
        let word = INTERESTING_WORDS[self.rng.gen_range(0..INTERESTING_WORDS.len())];
        input.bytes[index..index + 4].copy_from_slice(&word.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutations_are_deterministic_for_a_seed() {
        let mut a = Mutator::new(42);
        let mut b = Mutator::new(42);
        let mut ia = Input::from_vec(vec![0; 16]);
        let mut ib = Input::from_vec(vec![0; 16]);
        a.mutate(&mut ia, 64);
        b.mutate(&mut ib, 64);
        assert_eq!(ia, ib);
        assert_ne!(ia.as_slice(), &[0u8; 16]);
    }

    #[test]
    fn insert_respects_max_len() {
        let mut m = Mutator::new(7);
        let mut input = Input::from_vec(vec![0u8; 8]);
        for _ in 0..100 {
            m.mutate(&mut input, 8);
        }
        assert!(input.len() <= 8);
    }

    #[test]
    fn empty_inputs_gain_a_byte() {
        let mut m = Mutator::new(1);
        let mut input = Input::new();
        m.mutate(&mut input, 16);
        assert!(!input.is_empty());
    }
}
