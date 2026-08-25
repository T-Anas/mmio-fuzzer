//! Decoding test for `nop`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_nop() {
    let inst = decode16(0x46c0);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "nop decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Nop"),
        "expected Nop in {debug}"
    );
}
