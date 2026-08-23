//! Decoding test for `lsls_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_lsls_reg() {
    let inst = decode16(0x4088);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "lsls_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Lsl"),
        "expected Lsl in {debug}"
    );
}
