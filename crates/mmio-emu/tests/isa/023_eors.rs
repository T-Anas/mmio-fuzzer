//! Decoding test for `eors`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_eors() {
    let inst = decode16(0x4048);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "eors decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Eor"),
        "expected Eor in {debug}"
    );
}
