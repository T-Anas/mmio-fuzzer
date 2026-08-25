//! Decoding test for `muls`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_muls() {
    let inst = decode16(0x4348);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "muls decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Mul"),
        "expected Mul in {debug}"
    );
}
