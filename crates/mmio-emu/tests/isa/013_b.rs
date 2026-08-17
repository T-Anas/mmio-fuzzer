//! Decoding test for `b`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_b() {
    let inst = decode16(0xe7fe);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "b decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Branch"),
        "expected Branch in {debug}"
    );
}
