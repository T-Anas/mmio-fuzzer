//! Thumb decoding for ARMv6-M.
//!
//! The decoder is intentionally a separate pass from execution: it keeps
//! instruction semantics testable in isolation and makes it easy to record
//! which encodings we have already met while fuzzing.
//!
//! ARMv6-M decoding is a two-level affair. The top five bits select a group;
//! a handful of groups then look at one more bit (or a three-bit sub-opcode)
//! to pick the exact instruction. Matching on six bits instead is a classic
//! mistake, because for several groups that sixth bit is already part of an
//! operand field.

/// Immediate shift instructions (`LSLS`, `LSRS`, `ASRS #imm`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShiftKind {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

/// Shift amount: either an inline immediate or a register operand.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShiftAmount {
    Imm(u8),
    Reg(u8),
}

/// Data-processing operations in the register ALU group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AluOp {
    And,
    Eor,
    Lsl,
    Lsr,
    Asr,
    Adc,
    Sbc,
    Ror,
    Tst,
    Rsb,
    Cmp,
    Cmn,
    Orr,
    Mul,
    Bic,
    Mvn,
}

/// Operations available in the high-register group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HighOp {
    Add,
    Cmp,
    Mov,
}

/// Sign/zero extension variants in the miscellaneous group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExtendKind {
    Sxth,
    Sxtb,
    Uxth,
    Uxtb,
}

/// Transfer width for load/store instructions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemSize {
    Byte,
    Half,
    Word,
}

/// Base register for an address computation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BaseReg {
    Reg(u8),
    Sp,
    Pc,
}

/// Effective-address offset: register or immediate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Offset {
    Imm(u32),
    Reg(u8),
}

/// Barrier variants reachable from a 32-bit encoding.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BarrierKind {
    Dsb,
    Dmb,
    Isb,
}

/// A decoded instruction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Inst {
    Shift {
        kind: ShiftKind,
        rd: u8,
        rm: u8,
        amount: ShiftAmount,
    },
    /// `ADDS`/`SUBS Rd, Rn, #imm3`.
    AddSubImm {
        sub: bool,
        rd: u8,
        rn: u8,
        imm: u32,
        set_flags: bool,
    },
    /// `ADDS`/`SUBS Rd, Rn, Rm`.
    AddSubReg {
        sub: bool,
        rd: u8,
        rn: u8,
        rm: u8,
        set_flags: bool,
    },
    /// `MOVS Rd, #imm8`.
    MovImm8 {
        rd: u8,
        imm: u32,
    },
    /// `CMP Rn, #imm8`.
    CmpImm8 {
        rn: u8,
        imm: u32,
    },
    /// `ADDS`/`SUBS Rd, #imm8`.
    AddSubImm8 {
        sub: bool,
        rd: u8,
        imm: u32,
    },
    Alu {
        op: AluOp,
        rd: u8,
        rm: u8,
    },
    High {
        op: HighOp,
        rd: u8,
        rm: u8,
    },
    Extend {
        kind: ExtendKind,
        rd: u8,
        rm: u8,
    },
    Bx {
        rm: u8,
        link: bool,
    },
    /// `ADR` family: `ADD Rd, PC/SP, #imm`.
    Adr {
        rd: u8,
        from_sp: bool,
        imm: u32,
    },
    LoadStore {
        load: bool,
        size: MemSize,
        signed: bool,
        rt: u8,
        base: BaseReg,
        offset: Offset,
    },
    LoadStoreMulti {
        load: bool,
        rn: u8,
        regs: u16,
    },
    Branch {
        cond: u8,
        offset: i32,
    },
    BranchLink {
        offset: i32,
    },
    Push {
        regs: u16,
        lr: bool,
    },
    Pop {
        regs: u16,
        pc: bool,
    },
    AddSp {
        imm: i32,
    },
    SubSp {
        imm: i32,
    },
    Cps {
        disable: bool,
    },
    Svc {
        imm: u8,
    },
    Bkpt {
        imm: u8,
    },
    Barrier(BarrierKind),
    Mrs {
        rd: u8,
        sysm: u8,
    },
    Msr {
        sysm: u8,
        rn: u8,
    },
    Nop,
    /// A 32-bit encoding we do not (yet) model, kept for diagnostics.
    Unsupported(u32),
}

#[inline]
fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value << shift) as i32) >> shift
}

/// Decodes a single 16-bit Thumb instruction.
pub fn decode16(hw: u16) -> Inst {
    let b = |n: u32| ((hw as u32 >> n) & 1) as u8;
    let f = |hi: u32, lo: u32| (hw as u32 >> lo) & ((1 << (hi - lo + 1)) - 1);

    match hw >> 11 {
        // --- Shift (immediate) -------------------------------------------------
        0b00000..=0b00010 => {
            let kind = match hw >> 11 {
                0b00000 => ShiftKind::Lsl,
                0b00001 => ShiftKind::Lsr,
                _ => ShiftKind::Asr,
            };
            Inst::Shift {
                kind,
                rd: f(2, 0) as u8,
                rm: f(5, 3) as u8,
                amount: ShiftAmount::Imm(f(10, 6) as u8),
            }
        }

        // --- ADD/SUB register or 3-bit immediate -------------------------------
        0b00011 => {
            let sub = b(9) != 0;
            let rd = f(2, 0) as u8;
            let rn = f(5, 3) as u8;
            if b(10) != 0 {
                Inst::AddSubImm {
                    sub,
                    rd,
                    rn,
                    imm: f(8, 6),
                    set_flags: true,
                }
            } else {
                Inst::AddSubReg {
                    sub,
                    rd,
                    rn,
                    rm: f(8, 6) as u8,
                    set_flags: true,
                }
            }
        }

        // --- MOV/CMP/ADD/SUB 8-bit immediate -----------------------------------
        0b00100 => Inst::MovImm8 {
            rd: f(10, 8) as u8,
            imm: f(7, 0),
        },
        0b00101 => Inst::CmpImm8 {
            rn: f(10, 8) as u8,
            imm: f(7, 0),
        },
        0b00110 => Inst::AddSubImm8 {
            sub: false,
            rd: f(10, 8) as u8,
            imm: f(7, 0),
        },
        0b00111 => Inst::AddSubImm8 {
            sub: true,
            rd: f(10, 8) as u8,
            imm: f(7, 0),
        },

        // --- Data-processing register / special data ---------------------------
        0b01000 => {
            if b(10) == 0 {
                let op = match f(9, 6) {
                    0b0000 => AluOp::And,
                    0b0001 => AluOp::Eor,
                    0b0010 => AluOp::Lsl,
                    0b0011 => AluOp::Lsr,
                    0b0100 => AluOp::Asr,
                    0b0101 => AluOp::Adc,
                    0b0110 => AluOp::Sbc,
                    0b0111 => AluOp::Ror,
                    0b1000 => AluOp::Tst,
                    0b1001 => AluOp::Rsb,
                    0b1010 => AluOp::Cmp,
                    0b1011 => AluOp::Cmn,
                    0b1100 => AluOp::Orr,
                    0b1101 => AluOp::Mul,
                    0b1110 => AluOp::Bic,
                    _ => AluOp::Mvn,
                };
                Inst::Alu {
                    op,
                    rd: f(2, 0) as u8,
                    rm: f(5, 3) as u8,
                }
            } else {
                match f(9, 8) {
                    0b0000..=0b0010 => Inst::High {
                        op: match f(9, 8) {
                            0b0000 => HighOp::Add,
                            0b0001 => HighOp::Cmp,
                            _ => HighOp::Mov,
                        },
                        rd: (f(2, 0) as u8) | (b(7) << 3),
                        rm: (f(5, 3) as u8) | (b(6) << 3),
                    },
                    0b0011 => Inst::Bx {
                        rm: f(6, 3) as u8,
                        link: b(7) != 0,
                    },
                    _ => Inst::Unsupported(hw as u32),
                }
            }
        }

        // --- PC-relative literal load ------------------------------------------
        0b01001 => Inst::LoadStore {
            load: true,
            size: MemSize::Word,
            signed: false,
            rt: f(10, 8) as u8,
            base: BaseReg::Pc,
            offset: Offset::Imm(f(7, 0) * 4),
        },

        // --- Load/store register offset ----------------------------------------
        0b01010 | 0b01011 => {
            let (load, size, signed) = match f(11, 9) {
                0b000 => (false, MemSize::Word, false),
                0b001 => (false, MemSize::Half, false),
                0b010 => (false, MemSize::Byte, false),
                0b011 => (true, MemSize::Byte, true),
                0b100 => (true, MemSize::Word, false),
                0b101 => (true, MemSize::Half, false),
                0b110 => (true, MemSize::Byte, false),
                _ => (true, MemSize::Half, true),
            };
            Inst::LoadStore {
                load,
                size,
                signed,
                rt: f(2, 0) as u8,
                base: BaseReg::Reg(f(5, 3) as u8),
                offset: Offset::Reg(f(8, 6) as u8),
            }
        }

        // --- Load/store immediate word -----------------------------------------
        0b01100 => str_or_ldr(false, MemSize::Word, hw),
        0b01101 => str_or_ldr(true, MemSize::Word, hw),

        // --- Load/store immediate byte -----------------------------------------
        0b01110 => str_or_ldr(false, MemSize::Byte, hw),
        0b01111 => str_or_ldr(true, MemSize::Byte, hw),

        // --- Load/store immediate halfword -------------------------------------
        0b10000 => str_or_ldr(false, MemSize::Half, hw),
        0b10001 => str_or_ldr(true, MemSize::Half, hw),

        // --- SP-relative load/store --------------------------------------------
        0b10010..=0b10011 => Inst::LoadStore {
            load: b(11) != 0,
            size: MemSize::Word,
            signed: false,
            rt: f(10, 8) as u8,
            base: BaseReg::Sp,
            offset: Offset::Imm(f(7, 0) * 4),
        },

        // --- ADR / ADD to SP ---------------------------------------------------
        0b10100..=0b10101 => Inst::Adr {
            rd: f(10, 8) as u8,
            from_sp: b(11) != 0,
            imm: f(7, 0) * 4,
        },

        // --- Miscellaneous -----------------------------------------------------
        0b10110 | 0b10111 => decode_misc(hw),

        // --- Multiple load/store -----------------------------------------------
        0b11000..=0b11001 => Inst::LoadStoreMulti {
            load: b(11) != 0,
            rn: f(10, 8) as u8,
            regs: f(7, 0) as u16,
        },

        // --- Conditional branch / SVC ------------------------------------------
        0b11010..=0b11011 => {
            if f(11, 8) == 0b1111 {
                Inst::Svc { imm: f(7, 0) as u8 }
            } else {
                Inst::Branch {
                    cond: f(11, 8) as u8,
                    offset: sign_extend(f(7, 0), 8) * 2,
                }
            }
        }

        // --- Unconditional branch ----------------------------------------------
        0b11100 => Inst::Branch {
            cond: 0b1110,
            offset: sign_extend(f(10, 0), 11) * 2,
        },

        _ => Inst::Unsupported(hw as u32),
    }
}

/// Shared decoder for the four immediate load/store shapes.
fn str_or_ldr(load: bool, size: MemSize, hw: u16) -> Inst {
    let f = |hi: u32, lo: u32| (hw as u32 >> lo) & ((1 << (hi - lo + 1)) - 1);
    let scale = match size {
        MemSize::Byte => 1,
        MemSize::Half => 2,
        MemSize::Word => 4,
    };
    Inst::LoadStore {
        load,
        size,
        signed: false,
        rt: f(2, 0) as u8,
        base: BaseReg::Reg(f(5, 3) as u8),
        offset: Offset::Imm(f(10, 6) * scale),
    }
}

/// Decodes the miscellaneous 16-bit group (`1011 xxxx`).
fn decode_misc(hw: u16) -> Inst {
    let b = |n: u32| ((hw as u32 >> n) & 1) as u8;
    let f = |hi: u32, lo: u32| (hw as u32 >> lo) & ((1 << (hi - lo + 1)) - 1);
    match f(11, 8) {
        // ADD/SUB SP, #imm7: bit 7 selects.
        0b0000..=0b0001 => {
            if b(7) == 0 {
                Inst::AddSp {
                    imm: f(6, 0) as i32 * 4,
                }
            } else {
                Inst::SubSp {
                    imm: f(6, 0) as i32 * 4,
                }
            }
        }
        0b0100..=0b0101 => Inst::Push {
            regs: f(7, 0) as u16,
            lr: b(8) != 0,
        },
        // Sign/zero extend: 1011 0010 op Rm Rd.
        0b0010 => {
            let kind = match f(7, 6) {
                0b00 => ExtendKind::Sxth,
                0b01 => ExtendKind::Sxtb,
                0b10 => ExtendKind::Uxth,
                _ => ExtendKind::Uxtb,
            };
            Inst::Extend {
                kind,
                rm: f(5, 3) as u8,
                rd: f(2, 0) as u8,
            }
        }
        // SETEND (bit4=0) or CPS (bit4=1). We only model the CPS forms.
        0b0110 => {
            if b(4) != 0 {
                Inst::Cps { disable: b(5) != 0 }
            } else {
                Inst::Unsupported(hw as u32)
            }
        }
        0b1100..=0b1101 => Inst::Pop {
            regs: f(7, 0) as u16,
            pc: b(8) != 0,
        },
        0b1110 => Inst::Bkpt { imm: f(7, 0) as u8 },
        0b1111 => Inst::Nop,
        _ => Inst::Unsupported(hw as u32),
    }
}

/// Decodes a 32-bit Thumb instruction from its two halfwords.
///
/// ARMv6-M only defines a small set of 32-bit encodings. Anything else is
/// reported as [`Inst::Unsupported`] so the fuzzer can record it as an
/// uncovered encoding instead of silently doing the wrong thing.
pub fn decode32(hw1: u16, hw2: u16) -> Inst {
    let b1 = |n: u32| ((hw1 as u32 >> n) & 1) as u8;
    let f1 = |hi: u32, lo: u32| (hw1 as u32 >> lo) & ((1 << (hi - lo + 1)) - 1);
    let f2 = |hi: u32, lo: u32| (hw2 as u32 >> lo) & ((1 << (hi - lo + 1)) - 1);

    // Branch with link: 11110 S imm10 ; 11 J1 1 J2 imm11
    if (hw1 & 0xF800) == 0xF000 && (hw2 & 0xC000) == 0xC000 {
        let s = b1(10) as u32;
        let j1 = f2(13, 13);
        let j2 = f2(11, 11);
        let i1 = (!(j1 ^ s)) & 1;
        let i2 = (!(j2 ^ s)) & 1;
        // imm10 is 10 bits; the sign bit lives at bit 10 of the first halfword.
        let imm = (s << 24) | (i1 << 23) | (i2 << 22) | (f2(10, 0) << 1) | (f1(9, 0) << 12);
        return Inst::BranchLink {
            offset: sign_extend(imm, 25),
        };
    }

    // Barriers and hints: 1111 0011 1011 1111 ; 1000...
    if hw1 == 0xF3BF {
        return match hw2 & 0xFF00 {
            0x8F40 => Inst::Barrier(BarrierKind::Dsb),
            0x8F50 => Inst::Barrier(BarrierKind::Dmb),
            0x8F60 => Inst::Barrier(BarrierKind::Isb),
            _ => Inst::Unsupported(((hw1 as u32) << 16) | hw2 as u32),
        };
    }

    // MRS Rd, spec_reg : 1111 0011 1110 1111 ; 1000 Rd(11..8)
    if hw1 == 0xF3EF && (hw2 & 0xF000) == 0x8000 {
        return Inst::Mrs {
            rd: f2(11, 8) as u8,
            sysm: f2(7, 0) as u8,
        };
    }

    // MSR spec_reg, Rn : 1111 0011 1000 Rn ; 1000 (0)(0) sysm
    if (hw1 & 0xFF00) == 0xF300 && (hw2 & 0xFF00) == 0x8800 {
        return Inst::Msr {
            rn: f1(3, 0) as u8,
            sysm: f2(7, 0) as u8,
        };
    }

    Inst::Unsupported(((hw1 as u32) << 16) | hw2 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_shift_immediate() {
        // LSLS r0, r1, #3  => 0000 0 00011 001 000 = 0x00C8
        let i = decode16(0x00C8);
        assert_eq!(
            i,
            Inst::Shift {
                kind: ShiftKind::Lsl,
                rd: 0,
                rm: 1,
                amount: ShiftAmount::Imm(3)
            }
        );
    }

    #[test]
    fn decodes_shift_variants_across_bit10() {
        // The top bit of imm5 must not be mistaken for an opcode bit.
        // LSRS r0, r1, #0x10 => 0000 1 10000 001 000 = 0x0C08
        assert_eq!(
            decode16(0x0C08),
            Inst::Shift {
                kind: ShiftKind::Lsr,
                rd: 0,
                rm: 1,
                amount: ShiftAmount::Imm(0x10)
            }
        );
    }

    #[test]
    fn decodes_add_registers() {
        // ADDS r0, r1, r2 => 0001 100 010 001 000 = 0x1888
        assert_eq!(
            decode16(0x1888),
            Inst::AddSubReg {
                sub: false,
                rd: 0,
                rn: 1,
                rm: 2,
                set_flags: true
            }
        );
    }

    #[test]
    fn decodes_add_immediate3() {
        // ADDS r0, r1, #3 => 0001 110 011 001 000 = 0x1CC8
        assert_eq!(
            decode16(0x1CC8),
            Inst::AddSubImm {
                sub: false,
                rd: 0,
                rn: 1,
                imm: 3,
                set_flags: true
            }
        );
    }

    #[test]
    fn decodes_mov_immediate() {
        // MOVS r0, #0x42 => 00100 000 01000010 = 0x2042
        assert_eq!(decode16(0x2042), Inst::MovImm8 { rd: 0, imm: 0x42 });
    }

    #[test]
    fn decodes_bx_lr() {
        // BX LR = 0x4770
        assert_eq!(
            decode16(0x4770),
            Inst::Bx {
                rm: 14,
                link: false
            }
        );
    }

    #[test]
    fn decodes_branch_link_backwards() {
        // BL #-4 encodes as F7FF FFFE (verified with arm-none-eabi-as).
        let i = decode32(0xF7FF, 0xFFFE);
        assert_eq!(i, Inst::BranchLink { offset: -4 });
    }

    #[test]
    fn decodes_push_lr() {
        // PUSH {r7, lr} = 0xB580
        assert_eq!(
            decode16(0xB580),
            Inst::Push {
                regs: 0x80,
                lr: true
            }
        );
    }

    #[test]
    fn decodes_pop_pc() {
        // POP {pc} = 0xBD00
        assert_eq!(decode16(0xBD00), Inst::Pop { regs: 0, pc: true });
    }

    #[test]
    fn decodes_multiple_load() {
        // LDMIA r0!, {r1, r2} = 0xC806
        assert_eq!(
            decode16(0xC806),
            Inst::LoadStoreMulti {
                load: true,
                rn: 0,
                regs: 0x06
            }
        );
    }

    #[test]
    fn decodes_nop() {
        assert_eq!(decode16(0xBF00), Inst::Nop);
    }

    #[test]
    fn decodes_extend_instructions() {
        // UXTH r1, r3 = 0xB299
        assert_eq!(
            decode16(0xB299),
            Inst::Extend {
                kind: ExtendKind::Uxth,
                rd: 1,
                rm: 3
            }
        );
        // UXTB r3, r3 = 0xB2DB
        assert_eq!(
            decode16(0xB2DB),
            Inst::Extend {
                kind: ExtendKind::Uxtb,
                rd: 3,
                rm: 3
            }
        );
    }

    #[test]
    fn decodes_sub_sp() {
        // SUB SP, #0x10 = 0xB084
        assert_eq!(decode16(0xB084), Inst::SubSp { imm: 0x10 });
    }

    #[test]
    fn sign_extension_is_signed() {
        assert_eq!(sign_extend(0x3ff, 11), 1023);
        assert_eq!(sign_extend(0x400, 11), -1024);
        assert_eq!(sign_extend(0x7f, 8), 127);
        assert_eq!(sign_extend(0xff, 8), -1);
    }
}
