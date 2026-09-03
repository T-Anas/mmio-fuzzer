//! Decoding test for `sxth`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_sxth() {
    let inst = decode16(0xb208);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sxth decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Sxth"),
        "expected Sxth in {debug}"
    );
}
