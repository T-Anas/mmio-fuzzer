//! Decoding test for `revsh`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_revsh() {
    let inst = mmio_emu::decode::decode16(0xbac8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "revsh decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Rev"), "expected Rev in {debug}");
}
