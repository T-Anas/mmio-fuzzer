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

#[path = "isa_exec/001_movs_imm.rs"]
mod m_001_movs_imm;
#[path = "isa_exec/002_movs_zero.rs"]
mod m_002_movs_zero;
#[path = "isa_exec/003_adds_reg.rs"]
mod m_003_adds_reg;
#[path = "isa_exec/004_adds_carry.rs"]
mod m_004_adds_carry;
#[path = "isa_exec/005_adds_imm8.rs"]
mod m_005_adds_imm8;
#[path = "isa_exec/006_subs_reg.rs"]
mod m_006_subs_reg;
#[path = "isa_exec/007_subs_borrow.rs"]
mod m_007_subs_borrow;
#[path = "isa_exec/008_subs_imm8.rs"]
mod m_008_subs_imm8;
#[path = "isa_exec/009_ands.rs"]
mod m_009_ands;
#[path = "isa_exec/010_orrs.rs"]
mod m_010_orrs;
#[path = "isa_exec/011_eors.rs"]
mod m_011_eors;
#[path = "isa_exec/012_bics.rs"]
mod m_012_bics;
#[path = "isa_exec/013_mvns.rs"]
mod m_013_mvns;
#[path = "isa_exec/014_muls.rs"]
mod m_014_muls;
#[path = "isa_exec/015_tst.rs"]
mod m_015_tst;
#[path = "isa_exec/016_cmp_imm.rs"]
mod m_016_cmp_imm;
#[path = "isa_exec/017_cmp_reg.rs"]
mod m_017_cmp_reg;
#[path = "isa_exec/018_cmn.rs"]
mod m_018_cmn;
#[path = "isa_exec/019_lsls_imm.rs"]
mod m_019_lsls_imm;
#[path = "isa_exec/020_lsrs_imm.rs"]
mod m_020_lsrs_imm;
#[path = "isa_exec/021_asrs_imm.rs"]
mod m_021_asrs_imm;
#[path = "isa_exec/022_lsls_reg.rs"]
mod m_022_lsls_reg;
#[path = "isa_exec/023_lsrs_reg.rs"]
mod m_023_lsrs_reg;
#[path = "isa_exec/024_asrs_reg.rs"]
mod m_024_asrs_reg;
#[path = "isa_exec/025_rors.rs"]
mod m_025_rors;
#[path = "isa_exec/026_adcs.rs"]
mod m_026_adcs;
#[path = "isa_exec/027_sbcs.rs"]
mod m_027_sbcs;
#[path = "isa_exec/028_rsbs.rs"]
mod m_028_rsbs;
#[path = "isa_exec/029_uxtb.rs"]
mod m_029_uxtb;
#[path = "isa_exec/030_uxth.rs"]
mod m_030_uxth;
#[path = "isa_exec/031_sxtb.rs"]
mod m_031_sxtb;
#[path = "isa_exec/032_sxth.rs"]
mod m_032_sxth;
#[path = "isa_exec/033_rev.rs"]
mod m_033_rev;
#[path = "isa_exec/034_rev16.rs"]
mod m_034_rev16;
#[path = "isa_exec/035_revsh.rs"]
mod m_035_revsh;
#[path = "isa_exec/036_mov_high.rs"]
mod m_036_mov_high;
#[path = "isa_exec/037_add_high.rs"]
mod m_037_add_high;
#[path = "isa_exec/038_add_sp_imm.rs"]
mod m_038_add_sp_imm;
#[path = "isa_exec/039_sub_sp_imm.rs"]
mod m_039_sub_sp_imm;
