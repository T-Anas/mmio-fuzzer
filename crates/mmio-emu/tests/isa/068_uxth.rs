//! Decoding test for `uxth`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_uxth() {
    let inst = mmio_emu::decode::decode16(0xb288);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "uxth decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Uxth"), "expected Uxth in {debug}");
}
