//! Decoding test for `mov_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_mov_imm() {
    let inst = decode16(0x2005);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "mov_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("MovImm8"),
        "expected MovImm8 in {debug}"
    );
}
