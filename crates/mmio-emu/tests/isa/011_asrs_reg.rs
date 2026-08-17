//! Decoding test for `asrs_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_asrs_reg() {
    let inst = decode16(0x4108);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "asrs_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Asr"),
        "expected Asr in {debug}"
    );
}
