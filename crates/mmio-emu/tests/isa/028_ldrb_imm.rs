//! Decoding test for `ldrb_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_ldrb_imm() {
    let inst = mmio_emu::decode::decode16(0x7848);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldrb_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
