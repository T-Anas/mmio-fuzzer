//! Building an emulated machine for a firmware image.
//!
//! Real firmware is linked at `0x0800_0000` with its vector table at the base
//! of flash, but the core fetches reset vectors from address zero. We bridge
//! that by aliasing the vector table down to `0x0000_0000`, exactly like the
//! hardware's boot alias.

use std::fmt;
use std::path::Path;

use mmio_core::{MemoryMap, MemoryRegion, Permissions, PhysAddr, RegionKind};
use mmio_emu::{CortexM, Cpu, FlatMemory, Image, Shadow};
use serde::{Deserialize, Serialize};

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

/// Where the machine's RAM lives.
///
/// The default is a single generous RAM window, which is fine for firmware
/// that only talks MMIO. For parser targets we instead describe a *tight*
/// layout: exact stack, heap and input buffers with holes between them. Any
/// access into a hole is unmapped, so an out-of-bounds pointer becomes a
/// detectable `GuardHit` instead of silently reading neighbouring RAM.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryLayout {
    /// Default RAM window, or `None` to omit it (use the explicit regions).
    pub ram: Option<(u32, u32)>,
    /// Stack region (base, size).
    pub stack: Option<(u32, u32)>,
    /// Heap region (base, size).
    pub heap: Option<(u32, u32)>,
    /// Fuzz-input blob region (base, size).
    pub input: Option<(u32, u32)>,
    /// Ranges deliberately left unmapped, reported as `GuardHit`.
    pub guards: Vec<(u32, u32)>,
    /// Poisoned ranges inside mapped memory, reported as `OutOfBounds`.
    pub redzones: Vec<(u32, u32)>,
}

impl Default for MemoryLayout {
    fn default() -> Self {
        Self {
            ram: Some((DEFAULT_RAM_BASE, DEFAULT_RAM_SIZE)),
            stack: None,
            heap: None,
            input: None,
            guards: Vec::new(),
            redzones: Vec::new(),
        }
    }
}

impl MemoryLayout {
    /// A layout with no default RAM: only explicitly listed regions exist.
    pub fn tight() -> Self {
        Self {
            ram: None,
            ..Self::default()
        }
    }

    pub fn with_stack(mut self, base: u32, size: u32) -> Self {
        self.stack = Some((base, size));
        self
    }

    /// Sets the ordinary RAM window.
    pub fn with_ram(mut self, base: u32, size: u32) -> Self {
        self.ram = Some((base, size));
        self
    }

    pub fn with_heap(mut self, base: u32, size: u32) -> Self {
        self.heap = Some((base, size));
        self
    }

    pub fn with_input(mut self, base: u32, size: u32) -> Self {
        self.input = Some((base, size));
        self
    }

    pub fn with_guard(mut self, base: u32, size: u32) -> Self {
        self.guards.push((base, size));
        self
    }

    pub fn with_redzone(mut self, base: u32, size: u32) -> Self {
        self.redzones.push((base, size));
        self
    }

    /// True when `addr` falls inside a configured guard hole.
    pub fn is_guard(&self, addr: u32) -> bool {
        self.guards
            .iter()
            .any(|(base, size)| addr >= *base && addr < base.wrapping_add(*size))
    }
}

fn insert_ram(map: &mut MemoryMap, name: &str, base: u32, size: u32) {
    if size == 0 {
        return;
    }
    try_insert(
        map,
        MemoryRegion::new(
            name,
            PhysAddr::new(base),
            size,
            RegionKind::Ram,
            Permissions::RW,
        ),
    );
}

/// Builds the bus for a firmware image under an explicit memory layout.
///
/// The firmware image is applied but the fuzz input is *not*: the input
/// changes every run, so [`Engine`](crate::Engine) loads it separately into the
/// region named by `layout.input`.
pub fn build_memory(firmware: &Firmware, layout: &MemoryLayout) -> FlatMemory {
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

    if let Some((base, size)) = layout.ram {
        insert_ram(&mut map, "ram", base, size);
    }
    if let Some((base, size)) = layout.stack {
        insert_ram(&mut map, "stack", base, size);
    }
    if let Some((base, size)) = layout.heap {
        insert_ram(&mut map, "heap", base, size);
    }
    if let Some((base, size)) = layout.input {
        insert_ram(&mut map, "input", base, size);
    }

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
    if !layout.redzones.is_empty() {
        let mut shadow = Shadow::new();
        for (base, size) in &layout.redzones {
            shadow.poison(*base, *size);
        }
        memory.set_shadow(shadow);
    }
    memory
}

/// Builds a core with the firmware loaded and the default layout.
pub fn build_core(firmware: &Firmware) -> CortexM<FlatMemory> {
    CortexM::new(Cpu::new(), build_memory(firmware, &MemoryLayout::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_firmware_maps_and_aliases() {
        let bytes = [0u8; 64];
        let fw = Firmware::from_bin(DEFAULT_RAM_BASE, &bytes);
        let memory = build_memory(&fw, &MemoryLayout::default());
        assert!(memory.map().find(PhysAddr::new(DEFAULT_RAM_BASE)).is_some());
        assert!(memory.map().find(PhysAddr::new(PERIPHERAL_BASE)).is_some());
        assert!(memory.map().find(PhysAddr::new(SYSTEM_BASE)).is_some());
    }

    #[test]
    fn tight_layout_leaves_guards_unmapped() {
        let bytes = [0u8; 64];
        let fw = Firmware::from_bin(0x0800_0000, &bytes);
        let layout = MemoryLayout::tight()
            .with_stack(0x2000_0000, 0x1000)
            .with_heap(0x2000_2000, 0x1000)
            .with_input(0x2000_4000, 0x1000)
            .with_guard(0x2000_1000, 0x1000);
        let memory = build_memory(&fw, &layout);
        assert!(memory.map().find(PhysAddr::new(0x2000_0000)).is_some());
        assert!(memory.map().find(PhysAddr::new(0x2000_2000)).is_some());
        assert!(memory.map().find(PhysAddr::new(0x2000_4000)).is_some());
        // The guard hole is not mapped.
        assert!(memory.map().find(PhysAddr::new(0x2000_1000)).is_none());
        assert!(layout.is_guard(0x2000_1000));
        assert!(!layout.is_guard(0x2000_3000));
    }

    #[test]
    fn align_up_rounds() {
        assert_eq!(align_up(1, 4), 4);
        assert_eq!(align_up(4, 4), 4);
        assert_eq!(align_up(5, 4), 8);
    }
}
