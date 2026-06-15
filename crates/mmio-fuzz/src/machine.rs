//! Building an emulated machine for a firmware image.
//!
//! Real firmware is linked at `0x0800_0000` with its vector table at the base
//! of flash, but the core fetches reset vectors from address zero. We bridge
//! that by aliasing the vector table down to `0x0000_0000`, exactly like the
//! hardware's boot alias.

use std::fmt;
use std::path::Path;

use mmio_core::{MemoryMap, MemoryRegion, Permissions, PhysAddr, RegionKind};
use mmio_emu::{CortexM, Cpu, FlatMemory, Image};

/// Base of the ARMv6-M peripheral window.
pub const PERIPHERAL_BASE: u32 = 0x4000_0000;
/// Size of the peripheral window (0x4000_0000 .. 0x6000_0000).
pub const PERIPHERAL_SIZE: u32 = 0x2000_0000;
/// Base of the private peripheral bus.
pub const SYSTEM_BASE: u32 = 0xE000_0000;
/// Size of the private peripheral bus.
pub const SYSTEM_SIZE: u32 = 0x0010_0000;
/// Conventional SRAM base.
pub const DEFAULT_RAM_BASE: u32 = 0x2000_0000;
/// Stack/heap window we guarantee exists.
pub const DEFAULT_RAM_SIZE: u32 = 0x0002_0000;

/// Errors while setting up a machine.
#[derive(Debug)]
pub enum MachineError {
    Io(std::io::Error),
    Load(mmio_emu::LoadError),
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MachineError::Io(e) => write!(f, "io error: {e}"),
            MachineError::Load(e) => write!(f, "load error: {e}"),
        }
    }
}

impl std::error::Error for MachineError {}

impl From<std::io::Error> for MachineError {
    fn from(value: std::io::Error) -> Self {
        MachineError::Io(value)
    }
}

impl From<mmio_emu::LoadError> for MachineError {
    fn from(value: mmio_emu::LoadError) -> Self {
        MachineError::Load(value)
    }
}

/// A firmware image plus its content hash.
#[derive(Clone)]
pub struct Firmware {
    pub image: Image,
    pub hash: [u8; 32],
}

impl Firmware {
    pub fn from_elf_bytes(bytes: &[u8]) -> Result<Self, MachineError> {
        let image = Image::from_elf(bytes)?;
        Ok(Self {
            image,
            hash: *blake3::hash(bytes).as_bytes(),
        })
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, MachineError> {
        let bytes = std::fs::read(path)?;
        Self::from_elf_bytes(&bytes)
    }

    /// Raw binary placed at an explicit base address.
    pub fn from_bin(base: u32, bytes: &[u8]) -> Self {
        Self {
            image: Image::from_bin(base, bytes),
            hash: *blake3::hash(bytes).as_bytes(),
        }
    }

    pub fn entry(&self) -> u32 {
        self.image.entry
    }

    pub fn hash_hex(&self) -> String {
        self.hash[..8].iter().map(|b| format!("{b:02x}")).collect()
    }
}

fn try_insert(map: &mut MemoryMap, region: MemoryRegion) {
    // Overlapping default regions are normal (firmware often defines its own
    // RAM); we keep whatever is already mapped.
    let _ = map.insert(region);
}

fn align_up(value: u32, align: u32) -> u32 {
    value.div_ceil(align) * align
}

/// Picks the first `count` bytes of the lowest segment, for vector aliasing.
fn vector_bytes(firmware: &Firmware, count: usize) -> Vec<u8> {
    let Some(segment) = firmware.image.segments.iter().min_by_key(|s| s.addr) else {
        return Vec::new();
    };
    if segment.addr == 0 || segment.bytes.is_empty() {
        return Vec::new();
    }
    segment.bytes.iter().copied().take(count).collect()
}

/// Builds the bus for a firmware image, image already applied.
pub fn build_memory(firmware: &Firmware) -> FlatMemory {
    let mut map = MemoryMap::new();

    for segment in &firmware.image.segments {
        if segment.bytes.is_empty() {
            continue;
        }
        let is_flash = segment.addr < DEFAULT_RAM_BASE;
        let (kind, perms) = if is_flash {
            (RegionKind::Flash, Permissions::RX)
        } else {
            (RegionKind::Ram, Permissions::RW)
        };
        let size = align_up(segment.bytes.len().max(4) as u32, 4);
        try_insert(
            &mut map,
            MemoryRegion::new(
                format!("seg@{:#010x}", segment.addr),
                PhysAddr::new(segment.addr),
                size,
                kind,
                perms,
            ),
        );
    }

    try_insert(
        &mut map,
        MemoryRegion::new(
            "ram",
            PhysAddr::new(DEFAULT_RAM_BASE),
            DEFAULT_RAM_SIZE,
            RegionKind::Ram,
            Permissions::RW,
        ),
    );
    try_insert(
        &mut map,
        MemoryRegion::new(
            "peripheral",
            PhysAddr::new(PERIPHERAL_BASE),
            PERIPHERAL_SIZE,
            RegionKind::Peripheral,
            Permissions::RW,
        ),
    );
    try_insert(
        &mut map,
        MemoryRegion::new(
            "ppb",
            PhysAddr::new(SYSTEM_BASE),
            SYSTEM_SIZE,
            RegionKind::System,
            Permissions::RW,
        ),
    );

    let vectors = vector_bytes(firmware, 0x100);
    let alias = map.find(PhysAddr::ZERO).is_none() && !vectors.is_empty();
    if alias {
        try_insert(
            &mut map,
            MemoryRegion::new(
                "vectors",
                PhysAddr::ZERO,
                0x400,
                RegionKind::Flash,
                Permissions::RX,
            ),
        );
    }

    let mut memory = FlatMemory::new(map);
    let _ = firmware.image.apply(&mut memory);
    if alias {
        let _ = memory.load_image(PhysAddr::ZERO, &vectors);
    }
    memory
}

/// Builds a core with the firmware loaded but not yet reset.
pub fn build_core(firmware: &Firmware) -> CortexM<FlatMemory> {
    CortexM::new(Cpu::new(), build_memory(firmware))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_firmware_maps_and_aliases() {
        let bytes = [0u8; 64];
        let fw = Firmware::from_bin(DEFAULT_RAM_BASE, &bytes);
        let memory = build_memory(&fw);
        assert!(memory.map().find(PhysAddr::new(DEFAULT_RAM_BASE)).is_some());
        assert!(memory.map().find(PhysAddr::new(PERIPHERAL_BASE)).is_some());
        assert!(memory.map().find(PhysAddr::new(SYSTEM_BASE)).is_some());
    }

    #[test]
    fn align_up_rounds() {
        assert_eq!(align_up(1, 4), 4);
        assert_eq!(align_up(4, 4), 4);
        assert_eq!(align_up(5, 4), 8);
    }
}
