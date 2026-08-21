//! Decoding test for `ldr_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_ldr_imm() {
    let inst = decode16(0x6848);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldr_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("LoadStore"),
        "expected LoadStore in {debug}"
    );
}
