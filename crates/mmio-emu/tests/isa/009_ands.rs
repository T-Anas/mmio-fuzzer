//! Decoding test for `ands`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_ands() {
    let inst = decode16(0x4008);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ands decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("And"),
        "expected And in {debug}"
    );
}
