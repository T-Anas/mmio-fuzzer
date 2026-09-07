//! Decoding test for `strb_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_strb_imm() {
    let inst = mmio_emu::decode::decode16(0x7048);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "strb_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
