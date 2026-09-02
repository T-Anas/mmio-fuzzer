//! Decoding test for `uxtb`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_uxtb() {
    let inst = decode16(0xb2c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "uxtb decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Uxtb"),
        "expected Uxtb in {debug}"
    );
}
