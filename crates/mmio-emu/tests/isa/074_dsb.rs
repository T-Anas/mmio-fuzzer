//! Decoding test for `dsb`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_dsb() {
    let inst = mmio_emu::decode::decode32(0xf3bf, 0x8f4f);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "dsb decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Barrier"), "expected Barrier in {debug}");
}
