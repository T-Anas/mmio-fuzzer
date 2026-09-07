//! Decoding test for `add_high`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_add_high() {
    let inst = mmio_emu::decode::decode16(0x4440);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "add_high decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Add"), "expected Mov in {debug}");
}
