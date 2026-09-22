//! One execution test per instruction, assembled with `arm-none-eabi-as`.
//!
//! Each module runs a single instruction with fixed register inputs and checks
//! the architectural result.

pub mod harness {
    use mmio_core::{MemoryMap, MemoryRegion, Permissions, PhysAddr, RegionKind};
    use mmio_emu::{CortexM, Cpu, FlatMemory};

    /// Loads `program` at zero and steps it `steps` times after `setup`.
    pub fn run_with<F: FnOnce(&mut Cpu)>(
        program: &[u8],
        setup: F,
        steps: u64,
    ) -> CortexM<FlatMemory> {
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
            "ram",
            PhysAddr::new(0x2000_0000),
            0x1000,
            RegionKind::Ram,
            Permissions::RW,
        ))
        .unwrap();
        let mut memory = FlatMemory::new(map);
        memory.load_image(PhysAddr::ZERO, program).unwrap();
        let mut cpu = Cpu::new();
        cpu.msp = 0x2000_0800;
        setup(&mut cpu);
        let mut core = CortexM::new(cpu, memory);
        for _ in 0..steps {
            core.step().unwrap();
        }
        core
    }
}
