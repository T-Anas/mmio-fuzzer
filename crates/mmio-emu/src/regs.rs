//! Programmer's model: general-purpose registers, xPSR and control state.

/// Index of the stack pointer in the register file.
pub const SP: usize = 13;
/// Index of the link register in the register file.
pub const LR: usize = 14;
/// Index of the program counter in the register file.
pub const PC: usize = 15;

/// Number of general-purpose registers.
pub const NUM_REGS: usize = 16;

// --- xPSR bit positions -----------------------------------------------------

pub const APSR_N: u32 = 1 << 31;
pub const APSR_Z: u32 = 1 << 30;
pub const APSR_C: u32 = 1 << 29;
pub const APSR_V: u32 = 1 << 28;
pub const APSR_Q: u32 = 1 << 27;
/// EPSR.T is hard-wired to 1: Thumb only, there is no ARM state.
pub const EPSR_T: u32 = 1 << 24;
pub const IPSR_MASK: u32 = 0x1ff;

/// The combined program status register.
///
/// Storing it as raw bits keeps exception entry/exit trivial (save, remap,
/// restore) without a field-by-field dance.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Xpsr {
    bits: u32,
}

impl Xpsr {
    /// A fresh, reset xPSR with the Thumb bit set.
    pub const fn reset() -> Self {
        Self { bits: EPSR_T }
    }

    pub const fn from_bits(bits: u32) -> Self {
        Self {
            bits: bits | EPSR_T,
        }
    }

    pub const fn bits(self) -> u32 {
        self.bits
    }

    #[inline]
    pub const fn flag(self, mask: u32) -> bool {
        self.bits & mask != 0
    }

    #[inline]
    fn set(&mut self, mask: u32, value: bool) {
        if value {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    pub const fn n(self) -> bool {
        self.flag(APSR_N)
    }
    pub const fn z(self) -> bool {
        self.flag(APSR_Z)
    }
    pub const fn c(self) -> bool {
        self.flag(APSR_C)
    }
    pub const fn v(self) -> bool {
        self.flag(APSR_V)
    }

    pub fn set_n(&mut self, v: bool) {
        self.set(APSR_N, v);
    }
    pub fn set_z(&mut self, v: bool) {
        self.set(APSR_Z, v);
    }
    pub fn set_c(&mut self, v: bool) {
        self.set(APSR_C, v);
    }
    pub fn set_v(&mut self, v: bool) {
        self.set(APSR_V, v);
    }
    pub fn set_q(&mut self, v: bool) {
        self.set(APSR_Q, v);
    }

    /// Sets N and Z from a result, as most flag-setting instructions do.
    pub fn set_nz(&mut self, result: u32) {
        self.set_n(result & 0x8000_0000 != 0);
        self.set_z(result == 0);
    }

    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.set_n(n);
        self.set_z(z);
        self.set_c(c);
        self.set_v(v);
    }

    /// Vector number currently being serviced (0 in thread mode).
    pub const fn exception_number(self) -> u16 {
        (self.bits & IPSR_MASK) as u16
    }

    pub fn set_exception_number(&mut self, number: u16) {
        self.bits = (self.bits & !IPSR_MASK) | (number as u32 & IPSR_MASK);
    }
}

/// The CPU register file and special registers.
///
/// During execution `r[PC]` holds the address of the instruction being
/// executed, matching the architectural view for PC-relative operands. Whether
/// the instruction changed control flow is tracked by [`Cpu::branched`].
#[derive(Clone, Debug)]
pub struct Cpu {
    pub r: [u32; NUM_REGS],
    pub xpsr: Xpsr,
    /// Interrupt masking (PRIMASK).
    pub primask: bool,
    /// HardFault masking (FAULTMASK), present but rarely used on M0.
    pub faultmask: bool,
    /// CONTROL.SPSEL: false selects MSP, true selects PSP.
    pub spsel: bool,
    /// CONTROL.nPRIV: true when running unprivileged.
    pub npriv: bool,
    pub msp: u32,
    pub psp: u32,
    /// Set by a control-flow instruction during the current step.
    pub branched: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            r: [0; NUM_REGS],
            xpsr: Xpsr::reset(),
            primask: false,
            faultmask: false,
            spsel: false,
            npriv: false,
            msp: 0,
            psp: 0,
            branched: false,
        }
    }

    #[inline]
    pub const fn pc(&self) -> u32 {
        self.r[PC]
    }

    #[inline]
    pub fn set_pc(&mut self, value: u32) {
        self.r[PC] = value;
    }

    /// Architectural PC value seen by PC-relative operands.
    #[inline]
    pub const fn pc_relative(&self) -> u32 {
        self.r[PC].wrapping_add(4)
    }

    /// Performs a branch, remembering that control flow was redirected.
    #[inline]
    pub fn branch(&mut self, target: u32) {
        self.r[PC] = target & !1;
        self.branched = true;
    }

    /// Active stack pointer, honouring CONTROL.SPSEL.
    #[inline]
    pub const fn sp(&self) -> u32 {
        if self.spsel {
            self.psp
        } else {
            self.msp
        }
    }

    #[inline]
    pub fn set_sp(&mut self, value: u32) {
        if self.spsel {
            self.psp = value;
        } else {
            self.msp = value;
        }
    }

    /// Reads a register, with r13..r15 coming from their dedicated storage.
    #[inline]
    pub const fn read_reg(&self, index: usize) -> u32 {
        match index {
            SP => self.sp(),
            _ => self.r[index & 0xf],
        }
    }

    #[inline]
    pub fn write_reg(&mut self, index: usize, value: u32) {
        match index & 0xf {
            SP => self.set_sp(value),
            PC => self.branch(value),
            i => self.r[i] = value,
        }
    }

    /// Evaluates a 4-bit condition code against the current flags.
    pub fn condition_passed(&self, cond: u8) -> bool {
        let (n, z, c, v) = (self.xpsr.n(), self.xpsr.z(), self.xpsr.c(), self.xpsr.v());
        match cond & 0xf {
            0b0000 => z,
            0b0001 => !z,
            0b0010 => c,
            0b0011 => !c,
            0b0100 => n,
            0b0101 => !n,
            0b0110 => v,
            0b0111 => !v,
            0b1000 => c && !z,
            0b1001 => !c || z,
            0b1010 => n == v,
            0b1011 => n != v,
            0b1100 => !z && n == v,
            0b1101 => z || n != v,
            // 0b1110 is AL (always), 0b1111 is the "unconditional" escape.
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_sets_thumb_bit() {
        let psr = Xpsr::reset();
        assert!(psr.flag(EPSR_T));
        assert_eq!(psr.exception_number(), 0);
    }

    #[test]
    fn nz_from_result() {
        let mut psr = Xpsr::reset();
        psr.set_nz(0);
        assert!(psr.z() && !psr.n());
        psr.set_nz(0x8000_0000);
        assert!(psr.n() && !psr.z());
    }

    #[test]
    fn msp_psp_selection() {
        let mut cpu = Cpu::new();
        cpu.msp = 0x2000_1000;
        cpu.psp = 0x2000_2000;
        assert_eq!(cpu.sp(), 0x2000_1000);
        cpu.spsel = true;
        assert_eq!(cpu.sp(), 0x2000_2000);
        cpu.set_sp(0x2000_3000);
        assert_eq!(cpu.psp, 0x2000_3000);
        assert_eq!(cpu.msp, 0x2000_1000);
    }

    #[test]
    fn branch_clears_thumb_bit() {
        let mut cpu = Cpu::new();
        cpu.branch(0x0800_0001);
        assert_eq!(cpu.pc(), 0x0800_0000);
        assert!(cpu.branched);
    }

    #[test]
    fn conditions_behave() {
        let mut cpu = Cpu::new();
        cpu.xpsr.set_nzcv(false, true, true, false); // N=0, Z=1, C=1, V=0
        assert!(cpu.condition_passed(0b0000)); // EQ
        assert!(!cpu.condition_passed(0b0001)); // NE
        assert!(cpu.condition_passed(0b0010)); // CS
        assert!(!cpu.condition_passed(0b1000)); // HI is false while Z=1
        assert!(cpu.condition_passed(0b1001)); // LS is true while Z=1
        assert!(cpu.condition_passed(0b1110)); // AL
    }
}
