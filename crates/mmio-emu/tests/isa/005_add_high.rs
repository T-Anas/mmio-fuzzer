//! Decoding test for `add_high`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_add_high() {
    let inst = decode16(0x4440);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "add_high decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Mov"),
        "expected Mov in {debug}"
    );
}
