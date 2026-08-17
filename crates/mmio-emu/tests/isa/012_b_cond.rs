//! Decoding test for `b_cond`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_b_cond() {
    let inst = decode16(0xd0fe);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "b_cond decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Branch"),
        "expected Branch in {debug}"
    );
}
