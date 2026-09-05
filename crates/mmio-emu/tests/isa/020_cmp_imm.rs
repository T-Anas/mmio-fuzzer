//! Decoding test for `cmp_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_cmp_imm() {
    let inst = mmio_emu::decode::decode16(0x2803);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cmp_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("CmpImm8"), "expected CmpImm8 in {debug}");
}
