//! Decoding test for `tst`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_tst() {
    let inst = decode16(0x4208);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "tst decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Tst"),
        "expected Tst in {debug}"
    );
}
