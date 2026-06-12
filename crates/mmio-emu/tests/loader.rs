//! ELF loading against a real object file assembled for Cortex-M0.

use mmio_core::{AccessWidth, Bus, MemoryMap, MemoryRegion, Permissions, PhysAddr, RegionKind};
use mmio_emu::loader::Image;
use mmio_emu::FlatMemory;

const TINY: &[u8] = include_bytes!("data/tiny.elf");

#[test]
fn parses_segments_and_entry() {
    let image = Image::from_elf(TINY).expect("parse tiny.elf");
    // The entry point carries the Thumb bit.
    assert_eq!(image.entry & !1, 0x0800_0000);

    let seg = image
        .segments
        .iter()
        .find(|s| s.addr == 0x0800_0000)
        .expect("text segment");

    // Vector table: initial SP 0x2000_1000, then reset handler address.
    assert_eq!(&seg.bytes[..4], &[0x00, 0x10, 0x00, 0x20]);
    assert_eq!(seg.mem_size as usize, seg.bytes.len());
    assert_eq!(image.base(), Some(0x0800_0000));
}

#[test]
fn applies_image_to_memory() {
    let mut map = MemoryMap::new();
    map.insert(MemoryRegion::new(
        "flash",
        PhysAddr::new(0x0800_0000),
        0x1000,
        RegionKind::Flash,
        Permissions::RX,
    ))
    .unwrap();
    let mut mem = FlatMemory::new(map);

    let image = Image::from_elf(TINY).unwrap();
    image.apply(&mut mem).unwrap();

    assert_eq!(
        mem.read(PhysAddr::new(0x0800_0000), AccessWidth::Word).unwrap(),
        0x2000_1000
    );
}
