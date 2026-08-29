//! Decoding test for `rev`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_rev() {
    let inst = decode16(0xba08);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "rev decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Rev"),
        "expected Rev in {debug}"
    );
}
