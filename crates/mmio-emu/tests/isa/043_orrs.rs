//! Decoding test for `orrs`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_orrs() {
    let inst = decode16(0x4308);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "orrs decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Orr"),
        "expected Orr in {debug}"
    );
}
