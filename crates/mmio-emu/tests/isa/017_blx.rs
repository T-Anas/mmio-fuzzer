//! Decoding test for `blx`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_blx() {
    let inst = mmio_emu::decode::decode16(0x4788);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "blx decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Bx"), "expected Bx in {debug}");
}
