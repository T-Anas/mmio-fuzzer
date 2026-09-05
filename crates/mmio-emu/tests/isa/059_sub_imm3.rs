//! Decoding test for `sub_imm3`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_sub_imm3() {
    let inst = mmio_emu::decode::decode16(0x1ec8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sub_imm3 decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("AddSub"), "expected AddSub in {debug}");
}
