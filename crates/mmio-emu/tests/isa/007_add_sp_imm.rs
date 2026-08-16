//! Decoding test for `add_sp_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_add_sp_imm() {
    let inst = decode16(0xb004);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "add_sp_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("AddSp"),
        "expected AddSp in {debug}"
    );
}
