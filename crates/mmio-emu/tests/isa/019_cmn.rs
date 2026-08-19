//! Decoding test for `cmn`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_cmn() {
    let inst = decode16(0x42c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cmn decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Cmn"),
        "expected Cmn in {debug}"
    );
}
