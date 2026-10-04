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

//! Execution tests, included as one Cargo target.

#[path = "isa_exec/movs_imm.rs"]
mod m_001_movs_imm;
#[path = "isa_exec/movs_zero.rs"]
mod m_002_movs_zero;
#[path = "isa_exec/adds_reg.rs"]
mod m_003_adds_reg;
#[path = "isa_exec/adds_carry.rs"]
mod m_004_adds_carry;
#[path = "isa_exec/adds_imm8.rs"]
mod m_005_adds_imm8;
#[path = "isa_exec/subs_reg.rs"]
mod m_006_subs_reg;
#[path = "isa_exec/subs_borrow.rs"]
mod m_007_subs_borrow;
#[path = "isa_exec/subs_imm8.rs"]
mod m_008_subs_imm8;
#[path = "isa_exec/ands.rs"]
mod m_009_ands;
#[path = "isa_exec/orrs.rs"]
mod m_010_orrs;
#[path = "isa_exec/eors.rs"]
mod m_011_eors;
#[path = "isa_exec/bics.rs"]
mod m_012_bics;
#[path = "isa_exec/mvns.rs"]
mod m_013_mvns;
#[path = "isa_exec/muls.rs"]
mod m_014_muls;
#[path = "isa_exec/tst.rs"]
mod m_015_tst;
#[path = "isa_exec/cmp_imm.rs"]
mod m_016_cmp_imm;
#[path = "isa_exec/cmp_reg.rs"]
mod m_017_cmp_reg;
#[path = "isa_exec/cmn.rs"]
mod m_018_cmn;
#[path = "isa_exec/lsls_imm.rs"]
mod m_019_lsls_imm;
#[path = "isa_exec/lsrs_imm.rs"]
mod m_020_lsrs_imm;
#[path = "isa_exec/asrs_imm.rs"]
mod m_021_asrs_imm;
#[path = "isa_exec/lsls_reg.rs"]
mod m_022_lsls_reg;
#[path = "isa_exec/lsrs_reg.rs"]
mod m_023_lsrs_reg;
#[path = "isa_exec/asrs_reg.rs"]
mod m_024_asrs_reg;
#[path = "isa_exec/rors.rs"]
mod m_025_rors;
#[path = "isa_exec/adcs.rs"]
mod m_026_adcs;
#[path = "isa_exec/sbcs.rs"]
mod m_027_sbcs;
#[path = "isa_exec/rsbs.rs"]
mod m_028_rsbs;
#[path = "isa_exec/uxtb.rs"]
mod m_029_uxtb;
#[path = "isa_exec/uxth.rs"]
mod m_030_uxth;
#[path = "isa_exec/sxtb.rs"]
mod m_031_sxtb;
#[path = "isa_exec/sxth.rs"]
mod m_032_sxth;
#[path = "isa_exec/rev.rs"]
mod m_033_rev;
#[path = "isa_exec/rev16.rs"]
mod m_034_rev16;
#[path = "isa_exec/revsh.rs"]
mod m_035_revsh;
#[path = "isa_exec/mov_high.rs"]
mod m_036_mov_high;
#[path = "isa_exec/add_high.rs"]
mod m_037_add_high;
#[path = "isa_exec/add_sp_imm.rs"]
mod m_038_add_sp_imm;
#[path = "isa_exec/sub_sp_imm.rs"]
mod m_039_sub_sp_imm;
