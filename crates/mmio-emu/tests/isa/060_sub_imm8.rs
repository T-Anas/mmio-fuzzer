//! Decoding test for `sub_imm8`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_sub_imm8() {
    let inst = decode16(0x3807);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sub_imm8 decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("AddSub"),
        "expected AddSub in {debug}"
    );
}
