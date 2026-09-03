//! Decoding test for `yield`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_yield() {
    let inst = decode16(0xbf10);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "yield decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Nop"),
        "expected Nop in {debug}"
    );
}
