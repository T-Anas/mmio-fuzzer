//! Decoding test for `bkpt`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_bkpt() {
    let inst = decode16(0xbe00);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "bkpt decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Bkpt"),
        "expected Bkpt in {debug}"
    );
}
