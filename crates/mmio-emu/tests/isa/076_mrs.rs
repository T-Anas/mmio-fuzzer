//! Decoding test for `mrs`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_mrs() {
    let inst = decode32(0xf3ef, 0x8000);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "mrs decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Mrs"),
        "expected Mrs in {debug}"
    );
}
