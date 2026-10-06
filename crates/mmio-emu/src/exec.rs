//! Instruction execution.
//!
//! Each instruction follows the ARMv6-M pseudocode closely. Where the spec
//! defines flags, we set them; where it says "unchanged", we leave them alone.
//! Keeping that discipline is what lets real compiler output run unmodified.

use mmio_core::{AccessWidth, Bus, CoreError, PhysAddr, Result};

use crate::decode::{
    AluOp, BarrierKind, BaseReg, HighOp, Inst, MemSize, Offset, ShiftAmount, ShiftKind,
};
use crate::regs::{LR, PC};
use crate::CortexM;

/// EXC_RETURN value meaning "return to thread mode, use MSP".
pub const EXC_RETURN_THREAD_MSP: u32 = 0xFFFF_FFF9;
/// EXC_RETURN value meaning "return to thread mode, use PSP".
pub const EXC_RETURN_THREAD_PSP: u32 = 0xFFFF_FFFD;
/// High bits shared by every EXC_RETURN value.
pub const EXC_RETURN_MASK: u32 = 0xFFFF_FFF0;

const XPSR_N: u32 = 1 << 31;

impl<B: Bus> CortexM<B> {
    /// Executes one decoded instruction.
    pub(crate) fn exec(&mut self, inst: Inst) -> Result<()> {
        match inst {
            Inst::Shift {
                kind,
                rd,
                rm,
                amount,
            } => {
                let value = self.reg(rm);
                let amount = match amount {
                    ShiftAmount::Imm(v) => v as u32,
                    ShiftAmount::Reg(r) => self.reg(r) & 0xff,
                };
                let (result, carry, set_carry) = shift(kind, value, amount);
                self.cpu.write_reg(rd as usize, result);
                self.cpu.xpsr.set_nz(result);
                if set_carry {
                    self.cpu.xpsr.set_c(carry);
                }
            }

            Inst::AddSubImm {
                sub,
                rd,
                rn,
                imm,
                set_flags,
            } => {
                let a = self.reg(rn);
                let result = if sub {
                    self.sub_flags(a, imm, set_flags)
                } else {
                    self.add_flags(a, imm, false, set_flags)
                };
                self.cpu.write_reg(rd as usize, result);
            }

            Inst::AddSubReg {
                sub,
                rd,
                rn,
                rm,
                set_flags,
            } => {
                let a = self.reg(rn);
                let b = self.reg(rm);
                let result = if sub {
                    self.sub_flags(a, b, set_flags)
                } else {
                    self.add_flags(a, b, false, set_flags)
                };
                self.cpu.write_reg(rd as usize, result);
            }

            Inst::MovImm8 { rd, imm } => {
                self.cpu.write_reg(rd as usize, imm);
                self.cpu.xpsr.set_nz(imm);
            }

            Inst::CmpImm8 { rn, imm } => {
                let a = self.reg(rn);
                self.sub_flags(a, imm, true);
            }

            Inst::AddSubImm8 { sub, rd, imm } => {
                let a = self.reg(rd);
                let result = if sub {
                    self.sub_flags(a, imm, true)
                } else {
                    self.add_flags(a, imm, false, true)
                };
                self.cpu.write_reg(rd as usize, result);
            }

            Inst::Alu { op, rd, rm } => self.exec_alu(op, rd, rm),

            Inst::High { op, rd, rm } => {
                let b = self.reg(rm);
                match op {
                    HighOp::Add => {
                        let a = self.reg(rd);
                        self.cpu.write_reg(rd as usize, a.wrapping_add(b));
                    }
                    HighOp::Cmp => {
                        let a = self.reg(rd);
                        self.sub_flags(a, b, true);
                    }
                    HighOp::Mov => {
                        self.cpu.write_reg(rd as usize, b);
                    }
                }
            }

            Inst::Extend { kind, rd, rm } => {
                let value = self.reg(rm);
                let extended = match kind {
                    crate::decode::ExtendKind::Sxtb => (value as u8 as i8 as i32) as u32,
                    crate::decode::ExtendKind::Sxth => (value as u16 as i16 as i32) as u32,
                    crate::decode::ExtendKind::Uxtb => value & 0xff,
                    crate::decode::ExtendKind::Uxth => value & 0xffff,
                };
                self.cpu.write_reg(rd as usize, extended);
            }

            Inst::Rev { kind, rd, rm } => {
                let value = self.reg(rm);
                let reversed = match kind {
                    crate::decode::RevKind::Rev => value.swap_bytes(),
                    crate::decode::RevKind::Rev16 => {
                        let low = (value as u16).swap_bytes() as u32;
                        let high = ((value >> 16) as u16).swap_bytes() as u32;
                        low | (high << 16)
                    }
                    crate::decode::RevKind::Revsh => {
                        (value as u16).swap_bytes() as i16 as i32 as u32
                    }
                };
                self.cpu.write_reg(rd as usize, reversed);
            }

            Inst::Bx { rm, link } => {
                let target = self.reg(rm);
                if link {
                    let return_addr = self.cpu.pc().wrapping_add(2) | 1;
                    self.cpu.r[LR] = return_addr;
                }
                if target & EXC_RETURN_MASK == EXC_RETURN_MASK {
                    self.exception_return(target);
                } else {
                    self.cpu.branch(target);
                }
            }

            Inst::Adr { rd, from_sp, imm } => {
                let base = if from_sp {
                    self.cpu.sp()
                } else {
                    self.cpu.pc_relative() & !3
                };
                self.cpu.write_reg(rd as usize, base.wrapping_add(imm));
            }

            Inst::LoadStore {
                load,
                size,
                signed,
                rt,
                base,
                offset,
            } => {
                let addr = self.effective_address(base, offset);
                if load {
                    let width = width_of(size);
                    let raw = self.bus.read(addr, width)?;
                    let value = if signed {
                        sign_extend(raw, size)
                    } else {
                        raw & width.mask()
                    };
                    self.cpu.write_reg(rt as usize, value);
                } else {
                    let value = self.reg(rt);
                    self.bus.write(addr, width_of(size), value)?;
                }
            }

            Inst::LoadStoreMulti { load, rn, regs } => {
                let base = self.reg(rn);
                let count = regs.count_ones();
                if load {
                    for i in 0..8u32 {
                        if regs & (1 << i) != 0 {
                            let value = self
                                .bus
                                .read(PhysAddr::new(base.wrapping_add(i * 4)), AccessWidth::Word)?;
                            self.cpu.write_reg(i as usize, value);
                        }
                    }
                } else {
                    for i in 0..8u32 {
                        if regs & (1 << i) != 0 {
                            let value = self.cpu.read_reg(i as usize);
                            self.bus.write(
                                PhysAddr::new(base.wrapping_add(i * 4)),
                                AccessWidth::Word,
                                value,
                            )?;
                        }
                    }
                }
                self.cpu
                    .write_reg(rn as usize, base.wrapping_add(count * 4));
            }

            Inst::Branch { cond, offset } => {
                if self.cpu.condition_passed(cond) {
                    let target = self.cpu.pc_relative().wrapping_add(offset as u32);
                    self.cpu.branch(target);
                }
            }

            Inst::BranchLink { offset } => {
                let target = self.cpu.pc_relative().wrapping_add(offset as u32);
                let return_addr = self.cpu.pc_relative() | 1;
                self.cpu.r[LR] = return_addr;
                self.cpu.branch(target);
            }

            Inst::Push { regs, lr } => {
                let count = regs.count_ones() + lr as u32;
                let mut sp = self.cpu.sp().wrapping_sub(count * 4);
                self.cpu.set_sp(sp);
                for i in 0..8u32 {
                    if regs & (1 << i) != 0 {
                        let value = self.cpu.read_reg(i as usize);
                        self.bus
                            .write(PhysAddr::new(sp), AccessWidth::Word, value)?;
                        sp = sp.wrapping_add(4);
                    }
                }
                if lr {
                    let value = self.cpu.r[LR];
                    self.bus
                        .write(PhysAddr::new(sp), AccessWidth::Word, value)?;
                }
            }

            Inst::Pop { regs, pc } => {
                let mut sp = self.cpu.sp();
                for i in 0..8u32 {
                    if regs & (1 << i) != 0 {
                        let value = self.bus.read(PhysAddr::new(sp), AccessWidth::Word)?;
                        self.cpu.write_reg(i as usize, value);
                        sp = sp.wrapping_add(4);
                    }
                }
                if pc {
                    let value = self.bus.read(PhysAddr::new(sp), AccessWidth::Word)?;
                    sp = sp.wrapping_add(4);
                    if value & EXC_RETURN_MASK == EXC_RETURN_MASK {
                        self.exception_return(value);
                        return Ok(());
                    }
                    self.cpu.branch(value);
                }
                self.cpu.set_sp(sp);
            }

            Inst::AddSp { imm } => {
                let sp = self.cpu.sp().wrapping_add(imm as u32);
                self.cpu.set_sp(sp);
            }

            Inst::SubSp { imm } => {
                let sp = self.cpu.sp().wrapping_sub(imm as u32);
                self.cpu.set_sp(sp);
            }

            Inst::Cps { disable } => {
                self.cpu.primask = disable;
            }

            Inst::Svc { .. } => {
                self.take_exception(11); // SVCall
            }

            Inst::Bkpt { .. } => {
                self.take_exception(3); // HardFault
            }

            Inst::Barrier(kind) => {
                self.barrier(kind);
            }

            Inst::Mrs { rd, sysm } => {
                let value = self.read_special(sysm);
                self.cpu.write_reg(rd as usize, value);
            }

            Inst::Msr { sysm, rn } => {
                let value = self.reg(rn);
                self.write_special(sysm, value);
            }

            Inst::Nop => {}

            Inst::Unsupported(opcode) => {
                return Err(CoreError::InvalidOpcode {
                    opcode,
                    pc: self.cpu.pc(),
                });
            }
        }
        Ok(())
    }

    fn exec_alu(&mut self, op: AluOp, rd: u8, rm: u8) {
        let a = self.reg(rd);
        let b = self.reg(rm);
        match op {
            AluOp::And => {
                let r = a & b;
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Eor => {
                let r = a ^ b;
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Orr => {
                let r = a | b;
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Bic => {
                let r = a & !b;
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Mvn => {
                let r = !b;
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Tst => self.cpu.xpsr.set_nz(a & b),
            AluOp::Mul => {
                let r = a.wrapping_mul(b);
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
            }
            AluOp::Lsl | AluOp::Lsr | AluOp::Asr | AluOp::Ror => {
                let kind = match op {
                    AluOp::Lsl => ShiftKind::Lsl,
                    AluOp::Lsr => ShiftKind::Lsr,
                    AluOp::Asr => ShiftKind::Asr,
                    _ => ShiftKind::Ror,
                };
                let (r, carry, set_carry) = shift(kind, a, b & 0xff);
                self.cpu.write_reg(rd as usize, r);
                self.cpu.xpsr.set_nz(r);
                if set_carry {
                    self.cpu.xpsr.set_c(carry);
                }
            }
            AluOp::Adc => {
                let carry = self.cpu.xpsr.c();
                let r = self.add_flags(a, b, carry, true);
                self.cpu.write_reg(rd as usize, r);
            }
            AluOp::Sbc => {
                let carry_in = self.cpu.xpsr.c();
                let (r, carry, overflow) = sub_with_carry(a, b, carry_in);
                self.cpu.write_reg(rd as usize, r);
                self.cpu
                    .xpsr
                    .set_nzcv(r & XPSR_N != 0, r == 0, carry, overflow);
            }
            AluOp::Rsb => {
                let r = self.sub_flags(0, b, true);
                self.cpu.write_reg(rd as usize, r);
            }
            AluOp::Cmp => {
                self.sub_flags(a, b, true);
            }
            AluOp::Cmn => {
                self.add_flags(a, b, false, true);
            }
        }
    }

    /// `a + b (+ carry)`, updating NZCV when `set_flags` is true.
    fn add_flags(&mut self, a: u32, b: u32, carry_in: bool, set_flags: bool) -> u32 {
        let cin = carry_in as u64;
        let sum = a as u64 + b as u64 + cin;
        let result = sum as u32;
        if set_flags {
            let carry = sum >> 32 != 0;
            let overflow = ((a ^ result) & (b ^ result) & (1 << 31)) != 0;
            self.cpu
                .xpsr
                .set_nzcv(result & XPSR_N != 0, result == 0, carry, overflow);
        }
        result
    }

    /// `a - b`, updating NZCV when `set_flags` is true.
    fn sub_flags(&mut self, a: u32, b: u32, set_flags: bool) -> u32 {
        let (result, carry, overflow) = sub_with_borrow(a, b);
        if set_flags {
            self.cpu
                .xpsr
                .set_nzcv(result & XPSR_N != 0, result == 0, carry, overflow);
        }
        result
    }

    fn effective_address(&mut self, base: BaseReg, offset: Offset) -> PhysAddr {
        let base = match base {
            BaseReg::Reg(r) => self.reg(r),
            BaseReg::Sp => self.cpu.sp(),
            BaseReg::Pc => self.cpu.pc_relative() & !3,
        };
        let offset = match offset {
            Offset::Imm(v) => v,
            Offset::Reg(r) => self.reg(r),
        };
        PhysAddr::new(base.wrapping_add(offset))
    }

    #[inline]
    fn reg(&self, index: u8) -> u32 {
        if index as usize == PC {
            self.cpu.pc_relative()
        } else {
            self.cpu.read_reg(index as usize)
        }
    }

    fn barrier(&mut self, _kind: BarrierKind) {
        // Single-core interpreter: barriers order memory in ways that are
        // already guaranteed by the sequential model. Nothing to do.
    }

    fn read_special(&self, sysm: u8) -> u32 {
        match sysm {
            0..=3 => self.cpu.xpsr.bits(),
            5 => self.cpu.xpsr.exception_number() as u32,
            6 => self.cpu.xpsr.bits() & (1 << 24),
            8 => self.cpu.msp,
            9 => self.cpu.psp,
            16 => self.cpu.primask as u32,
            20 => (self.cpu.npriv as u32) | ((self.cpu.spsel as u32) << 1),
            _ => 0,
        }
    }

    fn write_special(&mut self, sysm: u8, value: u32) {
        match sysm {
            0 => self.cpu.xpsr = crate::regs::Xpsr::from_bits(value),
            8 => self.cpu.msp = value & !3,
            9 => self.cpu.psp = value & !3,
            16 => self.cpu.primask = value & 1 != 0,
            20 => {
                self.cpu.npriv = value & 1 != 0;
                self.cpu.spsel = value & 2 != 0;
            }
            _ => {}
        }
    }

    /// Minimal exception entry: stack the caller-saved frame and vector.
    pub(crate) fn take_exception(&mut self, exc: u16) {
        let frame_sp = self.cpu.sp().wrapping_sub(32);
        let frame = [
            self.cpu.r[0],
            self.cpu.r[1],
            self.cpu.r[2],
            self.cpu.r[3],
            self.cpu.r[12],
            self.cpu.r[LR],
            self.cpu.pc(),
            self.cpu.xpsr.bits(),
        ];
        for (i, value) in frame.iter().enumerate() {
            let _ = self.bus.write(
                PhysAddr::new(frame_sp + 4 * i as u32),
                AccessWidth::Word,
                *value,
            );
        }
        self.cpu.spsel = false;
        self.cpu.set_sp(frame_sp);
        self.cpu.r[LR] = EXC_RETURN_THREAD_MSP;
        self.cpu.xpsr.set_exception_number(exc);
        let vector = self
            .bus
            .read(PhysAddr::new((exc as u32) * 4), AccessWidth::Word)
            .unwrap_or(0);
        self.cpu.branch(vector & !1);
    }

    /// Handles a return from an exception (`BX LR` / `POP {pc}` with an
    /// EXC_RETURN value): pops the stacked frame and resumes thread execution.
    fn exception_return(&mut self, exc_return: u32) {
        let use_psp = exc_return & (1 << 2) != 0;
        let sp = if use_psp { self.cpu.psp } else { self.cpu.msp };
        let word = |cpu_bus: &mut B, offset: u32| {
            cpu_bus
                .read(PhysAddr::new(sp.wrapping_add(offset)), AccessWidth::Word)
                .unwrap_or(0)
        };
        let r0 = word(&mut self.bus, 0);
        let r1 = word(&mut self.bus, 4);
        let r2 = word(&mut self.bus, 8);
        let r3 = word(&mut self.bus, 12);
        let r12 = word(&mut self.bus, 16);
        let lr = word(&mut self.bus, 20);
        let pc = word(&mut self.bus, 24);
        let xpsr = word(&mut self.bus, 28);
        self.cpu.r[0] = r0;
        self.cpu.r[1] = r1;
        self.cpu.r[2] = r2;
        self.cpu.r[3] = r3;
        self.cpu.r[12] = r12;
        self.cpu.r[LR] = lr;
        self.cpu.xpsr = crate::regs::Xpsr::from_bits(xpsr);
        self.cpu.spsel = use_psp;
        let new_sp = sp.wrapping_add(32);
        if use_psp {
            self.cpu.psp = new_sp;
        } else {
            self.cpu.msp = new_sp;
        }
        self.cpu.branch(pc);
    }

    /// Loads SP and the reset vector from the vector table at address 0.
    pub fn reset(&mut self) -> Result<()> {
        let sp = self.bus.read(PhysAddr::ZERO, AccessWidth::Word)?;
        let reset = self.bus.read(PhysAddr::new(4), AccessWidth::Word)?;
        self.cpu.msp = sp & !7;
        self.cpu.spsel = false;
        self.cpu.set_sp(sp & !7);
        self.cpu.set_pc(reset & !1);
        self.cpu.xpsr = crate::regs::Xpsr::reset();
        Ok(())
    }
}

#[inline]
fn width_of(size: MemSize) -> AccessWidth {
    match size {
        MemSize::Byte => AccessWidth::Byte,
        MemSize::Half => AccessWidth::HalfWord,
        MemSize::Word => AccessWidth::Word,
    }
}

#[inline]
fn sign_extend(raw: u32, size: MemSize) -> u32 {
    match size {
        MemSize::Word => raw,
        MemSize::Half => (raw as u16 as i16 as i32) as u32,
        MemSize::Byte => (raw as u8 as i8 as i32) as u32,
    }
}

/// `a - b`, returning `(result, carry, overflow)` where carry is `!borrow`
/// and overflow reflects signed wrapping.
#[inline]
fn sub_with_borrow(a: u32, b: u32) -> (u32, bool, bool) {
    let result = a.wrapping_sub(b);
    let carry = a >= b;
    let overflow = ((a ^ b) & (a ^ result) & (1 << 31)) != 0;
    (result, carry, overflow)
}

/// `a - b - (1 - carry_in)`, used by `SBC`.
#[inline]
fn sub_with_carry(a: u32, b: u32, carry_in: bool) -> (u32, bool, bool) {
    let borrow_in = !carry_in as u32;
    let subtrahend = b as u64 + borrow_in as u64;
    let result = a.wrapping_sub(b).wrapping_sub(borrow_in);
    let carry = (a as u64) >= subtrahend;
    let overflow = ((a ^ b) & (a ^ result) & (1 << 31)) != 0;
    (result, carry, overflow)
}

/// Applies a shift, returning `(result, carry, carry_was_set)`.
fn shift(kind: ShiftKind, value: u32, amount: u32) -> (u32, bool, bool) {
    match kind {
        ShiftKind::Lsl => match amount {
            0 => (value, false, false),
            n if n < 32 => (value << n, (value >> (32 - n)) & 1 != 0, true),
            32 => (0, value & 1 != 0, true),
            _ => (0, false, true),
        },
        ShiftKind::Lsr => match amount {
            0 => (value, false, false),
            n if n < 32 => (value >> n, (value >> (n - 1)) & 1 != 0, true),
            32 => (0, (value >> 31) & 1 != 0, true),
            _ => (0, false, true),
        },
        ShiftKind::Asr => match amount {
            0 => (value, false, false),
            n => {
                let n = n.min(32);
                let carry = (value >> (n - 1)) & 1 != 0;
                (((value as i32) >> n) as u32, carry, true)
            }
        },
        ShiftKind::Ror => match amount {
            0 => (value, false, false),
            n => {
                let n = n & 31;
                if n == 0 {
                    (value, value >> 31 & 1 != 0, true)
                } else {
                    let r = value.rotate_right(n);
                    (r, r >> 31 & 1 != 0, true)
                }
            }
        },
    }
}
