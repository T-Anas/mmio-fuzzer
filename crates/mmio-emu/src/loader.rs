//! Loading firmware images into an emulated address space.
//!
//! We support ELF (the natural output of `arm-none-eabi-gcc`) and raw binary
//! blobs. Only loadable segments are relevant: on a bare-metal target the
//! interesting content is `.text`, `.rodata` and the initialised part of
//! `.data`. `.bss` is zeroed RAM and needs no file bytes.

use mmio_core::{PhysAddr, Result as CoreResult};

use crate::memory::FlatMemory;

/// One loadable segment, described independently of the file format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    /// Virtual (load) address.
    pub addr: u32,
    /// Bytes present in the file.
    pub bytes: Vec<u8>,
    /// Total memory footprint, which may exceed `bytes.len()` (`.bss`-like).
    pub mem_size: u32,
}

/// A parsed firmware image ready to be applied to a bus.
#[derive(Clone, Debug, Default)]
pub struct Image {
    /// Where execution starts. For Cortex-M this is typically the reset
    /// vector, i.e. the second word of the vector table.
    pub entry: u32,
    pub segments: Vec<Segment>,
}

/// Errors that can occur while parsing a firmware file.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("object parse error: {0}")]
    Parse(#[from] object::Error),
    #[error("image entry point {0:#x} does not fit in 32 bits")]
    EntryTooLarge(u64),
    #[error("segment at {addr:#x} is larger than 4 GiB")]
    SegmentTooLarge { addr: u64 },
}

impl Image {
    /// Parses an ELF image.
    pub fn from_elf(bytes: &[u8]) -> std::result::Result<Self, LoadError> {
        use object::{Object, ObjectSegment};

        let file = object::File::parse(bytes)?;
        let entry = file.entry();
        if entry > u32::MAX as u64 {
            return Err(LoadError::EntryTooLarge(entry));
        }

        let mut segments = Vec::new();
        for segment in file.segments() {
            let addr = segment.address();
            if addr > u32::MAX as u64 {
                return Err(LoadError::SegmentTooLarge { addr });
            }
            let data = segment.data()?;
            let mem_size = segment.size();
            segments.push(Segment {
                addr: addr as u32,
                bytes: data.to_vec(),
                mem_size: mem_size.min(u32::MAX as u64) as u32,
            });
        }
        segments.sort_by_key(|s| s.addr);
        Ok(Self { entry: entry as u32, segments })
    }

    /// Wraps a raw binary blob placed at `base`.
    pub fn from_bin(base: u32, bytes: &[u8]) -> Self {
        Self {
            entry: base,
            segments: vec![Segment {
                addr: base,
                bytes: bytes.to_vec(),
                mem_size: bytes.len().min(u32::MAX as usize) as u32,
            }],
        }
    }

    /// Copies every segment into the bus.
    ///
    /// `mem_size` beyond the file bytes is left as-is; callers that need
    /// zero-filled `.bss` should map those regions as RAM, which already
    /// starts zeroed.
    pub fn apply(&self, memory: &mut FlatMemory) -> CoreResult<()> {
        for segment in &self.segments {
            if segment.bytes.is_empty() {
                continue;
            }
            memory.load_image(PhysAddr::new(segment.addr), &segment.bytes)?;
        }
        Ok(())
    }

    /// Total number of file bytes across segments.
    pub fn file_size(&self) -> usize {
        self.segments.iter().map(|s| s.bytes.len()).sum()
    }

    /// The lowest load address, useful for sanity checks.
    pub fn base(&self) -> Option<u32> {
        self.segments.iter().map(|s| s.addr).min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_bin_roundtrip() {
        let img = Image::from_bin(0x0800_0000, &[1, 2, 3, 4]);
        assert_eq!(img.entry, 0x0800_0000);
        assert_eq!(img.base(), Some(0x0800_0000));
        assert_eq!(img.file_size(), 4);
        assert_eq!(img.segments[0].mem_size, 4);
    }
}
