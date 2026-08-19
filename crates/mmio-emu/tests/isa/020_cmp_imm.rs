//! Decoding test for `cmp_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_cmp_imm() {
    let inst = decode16(0x2803);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cmp_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("CmpImm8"),
        "expected CmpImm8 in {debug}"
    );
}
