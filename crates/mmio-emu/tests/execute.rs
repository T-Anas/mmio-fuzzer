//! End-to-end execution of assembled Cortex-M0 code.
//!
//! The byte image was produced with `arm-none-eabi-as -mcpu=cortex-m0` from:
//!
//! ```asm
//!   movs r0, #5
//!   movs r1, #7
//!   adds r0, r0, r1
//!   movs r2, #0x40
//!   lsls r2, r2, #24      @ r2 = 0x4000_0000
//!   str  r0, [r2]
//!   ldr  r3, [r2]
//! ```
//!
//! `r0` should end at 12, and `r3` should read back whatever the peripheral
//! model returned for the store.

use std::cell::RefCell;
use std::rc::Rc;

use mmio_core::{
    Access, AccessWidth, Bus, FnObserver, MemoryMap, MemoryRegion, Permissions, PhysAddr,
    RegionKind,
};
use mmio_emu::memory::{FlatMemory, MmioHandler};
use mmio_emu::{CortexM, Cpu};

const PROGRAM: &[u8] = &[
    0x05, 0x20, // movs r0, #5
    0x07, 0x21, // movs r1, #7
    0x40, 0x18, // adds r0, r0, r1
    0x40, 0x22, // movs r2, #0x40
    0x12, 0x06, // lsls r2, r2, #24
    0x10, 0x60, // str  r0, [r2]
    0x13, 0x68, // ldr  r3, [r2]
    0x00, 0xbe, // bkpt #0
];

struct Echo {
    last: u32,
}

impl MmioHandler for Echo {
    fn read(&mut self, _addr: PhysAddr, _width: AccessWidth) -> Option<u32> {
        Some(self.last)
    }

    fn write(&mut self, _addr: PhysAddr, _width: AccessWidth, value: u32) -> Option<()> {
        self.last = value;
        Some(())
    }
}

fn build() -> (FlatMemory, Rc<RefCell<Vec<Access>>>) {
    let mut map = MemoryMap::new();
    map.insert(MemoryRegion::new(
        "code",
        PhysAddr::new(0),
        0x1000,
        RegionKind::Flash,
        Permissions::RX,
    ))
    .unwrap();
    map.insert(MemoryRegion::new(
        "sram",
        PhysAddr::new(0x2000_0000),
        0x1000,
        RegionKind::Ram,
        Permissions::RW,
    ))
    .unwrap();
    map.insert(MemoryRegion::new(
        "gpio0",
        PhysAddr::new(0x4000_0000),
        0x1000,
        RegionKind::Peripheral,
        Permissions::RW,
    ))
    .unwrap();

    let mut mem = FlatMemory::new(map);
    mem.load_image(PhysAddr::ZERO, PROGRAM).unwrap();
    mem.set_mmio_handler(Echo { last: 0 });

    let seen = Rc::new(RefCell::new(Vec::new()));
    let sink = seen.clone();
    mem.set_observer(FnObserver(move |a: &Access| sink.borrow_mut().push(*a)));

    (mem, seen)
}

#[test]
fn runs_assembled_arithmetic_and_mmio() {
    let (mem, seen) = build();
    let mut cpu = Cpu::new();
    cpu.msp = 0x2000_1000;
    let mut core = CortexM::new(cpu, mem);

    for _ in 0..7 {
        core.step().unwrap();
    }

    assert_eq!(core.cpu.r[0], 12, "r0 should hold 5 + 7");
    assert_eq!(core.cpu.r[3], 12, "r3 should echo the value written to MMIO");

    let accesses = seen.borrow();
    let mmio: Vec<_> = accesses
        .iter()
        .filter(|a| a.addr.raw() == 0x4000_0000)
        .collect();
    assert!(
        mmio.iter().any(|a| a.kind.is_write()),
        "the store should have been observed"
    );
    assert!(
        mmio.iter().any(|a| a.kind.is_read()),
        "the load should have been observed"
    );
    assert!(mmio.iter().all(|a| a.pc != 0), "accesses carry a PC");
}

#[test]
fn load_image_is_little_endian() {
    let (mut mem, _) = build();
    assert_eq!(
        mem.read(PhysAddr::ZERO, AccessWidth::HalfWord).unwrap(),
        0x2005
    );
}
