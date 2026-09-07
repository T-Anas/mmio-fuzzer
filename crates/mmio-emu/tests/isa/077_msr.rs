//! Decoding test for `msr`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_msr() {
    let inst = decode32(0xf380, 0x8800);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "msr decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Msr"),
        "expected Msr in {debug}"
    );
}
