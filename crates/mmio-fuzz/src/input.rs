//! The fuzz input.
//!
//! An [`Input`] is an opaque byte string. Where those bytes end up is the
//! engine's business: with a peripheral model in place, they bias which of a
//! register's plausible values is returned, so mutating them steers the
//! firmware through different device interactions.

use serde::{Deserialize, Serialize};

/// A single fuzz input.
#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Input {
    pub bytes: Vec<u8>,
}

impl Input {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Stable content hash, used for corpus bookkeeping and testcase names.
    pub fn digest(&self) -> [u8; 32] {
        *blake3::hash(&self.bytes).as_bytes()
    }

    /// Short hex digest, convenient for filenames.
    pub fn short_hash(&self) -> String {
        let digest = self.digest();
        digest[..8].iter().map(|b| format!("{b:02x}")).collect()
    }
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input({} bytes, {})", self.bytes.len(), self.short_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digests_are_stable_and_distinct() {
        let a = Input::from_vec(vec![1, 2, 3]);
        let b = Input::from_vec(vec![1, 2, 3]);
        let c = Input::from_vec(vec![3, 2, 1]);
        assert_eq!(a.digest(), b.digest());
        assert_ne!(a.digest(), c.digest());
        assert_eq!(a.short_hash().len(), 16);
    }
}
