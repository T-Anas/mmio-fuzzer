//! One decoding test per ARMv6-M instruction.
//!
//! Encodings are produced by `arm-none-eabi-as`. Each page is included here
//! so Cargo compiles them as a single test target.

#[path = "isa/001_adcs.rs"]
mod m_001_adcs;
#[path = "isa/002_add_imm3.rs"]
mod m_002_add_imm3;
#[path = "isa/003_add_imm8.rs"]
mod m_003_add_imm8;
#[path = "isa/004_add_reg.rs"]
mod m_004_add_reg;
#[path = "isa/005_add_high.rs"]
mod m_005_add_high;
#[path = "isa/006_add_sp.rs"]
mod m_006_add_sp;
#[path = "isa/007_add_sp_imm.rs"]
mod m_007_add_sp_imm;
#[path = "isa/009_ands.rs"]
mod m_009_ands;
#[path = "isa/010_asrs_imm.rs"]
mod m_010_asrs_imm;
#[path = "isa/011_asrs_reg.rs"]
mod m_011_asrs_reg;
#[path = "isa/012_b_cond.rs"]
mod m_012_b_cond;
#[path = "isa/013_b.rs"]
mod m_013_b;
#[path = "isa/014_bic.rs"]
mod m_014_bic;
#[path = "isa/015_bkpt.rs"]
mod m_015_bkpt;
#[path = "isa/016_bl.rs"]
mod m_016_bl;
#[path = "isa/017_blx.rs"]
mod m_017_blx;
#[path = "isa/018_bx.rs"]
mod m_018_bx;
#[path = "isa/019_cmn.rs"]
mod m_019_cmn;
#[path = "isa/020_cmp_imm.rs"]
mod m_020_cmp_imm;
#[path = "isa/021_cmp_reg.rs"]
mod m_021_cmp_reg;
#[path = "isa/022_cpsid.rs"]
mod m_022_cpsid;
#[path = "isa/023_eors.rs"]
mod m_023_eors;
#[path = "isa/024_ldm.rs"]
mod m_024_ldm;
#[path = "isa/025_ldr_imm.rs"]
mod m_025_ldr_imm;
#[path = "isa/027_ldr_reg.rs"]
mod m_027_ldr_reg;
#[path = "isa/028_ldrb_imm.rs"]
mod m_028_ldrb_imm;
#[path = "isa/029_ldrb_reg.rs"]
mod m_029_ldrb_reg;
#[path = "isa/030_ldrh_imm.rs"]
mod m_030_ldrh_imm;
#[path = "isa/031_ldrh_reg.rs"]
mod m_031_ldrh_reg;
#[path = "isa/032_ldrsb.rs"]
mod m_032_ldrsb;
#[path = "isa/033_ldrsh.rs"]
mod m_033_ldrsh;
#[path = "isa/034_lsls_imm.rs"]
mod m_034_lsls_imm;
#[path = "isa/035_lsls_reg.rs"]
mod m_035_lsls_reg;
#[path = "isa/036_lsrs_imm.rs"]
mod m_036_lsrs_imm;
#[path = "isa/037_lsrs_reg.rs"]
mod m_037_lsrs_reg;
#[path = "isa/038_mov_imm.rs"]
mod m_038_mov_imm;
#[path = "isa/039_mov_reg.rs"]
mod m_039_mov_reg;
#[path = "isa/040_muls.rs"]
mod m_040_muls;
#[path = "isa/041_mvns.rs"]
mod m_041_mvns;
#[path = "isa/042_nop.rs"]
mod m_042_nop;
#[path = "isa/043_orrs.rs"]
mod m_043_orrs;
#[path = "isa/044_pop.rs"]
mod m_044_pop;
#[path = "isa/045_push.rs"]
mod m_045_push;
#[path = "isa/046_rev.rs"]
mod m_046_rev;
#[path = "isa/047_rev16.rs"]
mod m_047_rev16;
#[path = "isa/048_revsh.rs"]
mod m_048_revsh;
#[path = "isa/049_rors.rs"]
mod m_049_rors;
#[path = "isa/050_rsbs.rs"]
mod m_050_rsbs;
#[path = "isa/051_sbcs.rs"]
mod m_051_sbcs;
#[path = "isa/052_stm.rs"]
mod m_052_stm;
#[path = "isa/053_str_imm.rs"]
mod m_053_str_imm;
#[path = "isa/054_str_reg.rs"]
mod m_054_str_reg;
#[path = "isa/055_strb_imm.rs"]
mod m_055_strb_imm;
#[path = "isa/056_strb_reg.rs"]
mod m_056_strb_reg;
#[path = "isa/057_strh_imm.rs"]
mod m_057_strh_imm;
#[path = "isa/058_strh_reg.rs"]
mod m_058_strh_reg;
#[path = "isa/059_sub_imm3.rs"]
mod m_059_sub_imm3;
#[path = "isa/060_sub_imm8.rs"]
mod m_060_sub_imm8;
#[path = "isa/061_sub_reg.rs"]
mod m_061_sub_reg;
#[path = "isa/062_sub_sp.rs"]
mod m_062_sub_sp;
#[path = "isa/063_svc.rs"]
mod m_063_svc;
#[path = "isa/064_sxtb.rs"]
mod m_064_sxtb;
#[path = "isa/065_sxth.rs"]
mod m_065_sxth;
#[path = "isa/066_tst.rs"]
mod m_066_tst;
#[path = "isa/067_uxtb.rs"]
mod m_067_uxtb;
#[path = "isa/068_uxth.rs"]
mod m_068_uxth;
#[path = "isa/069_wfe.rs"]
mod m_069_wfe;
#[path = "isa/070_wfi.rs"]
mod m_070_wfi;
#[path = "isa/071_yield.rs"]
mod m_071_yield;
#[path = "isa/072_sev.rs"]
mod m_072_sev;
#[path = "isa/073_dmb.rs"]
mod m_073_dmb;
#[path = "isa/074_dsb.rs"]
mod m_074_dsb;
#[path = "isa/075_isb.rs"]
mod m_075_isb;
#[path = "isa/076_mrs.rs"]
mod m_076_mrs;
#[path = "isa/077_msr.rs"]
mod m_077_msr;
