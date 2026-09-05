//! Decoding test for `dmb`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_dmb() {
    let inst = mmio_emu::decode::decode32(0xf3bf, 0x8f5f);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "dmb decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Barrier"), "expected Barrier in {debug}");
}
