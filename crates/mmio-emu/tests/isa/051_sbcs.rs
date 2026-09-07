//! Decoding test for `sbcs`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_sbcs() {
    let inst = mmio_emu::decode::decode16(0x4188);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sbcs decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Sbc"), "expected Sbc in {debug}");
}
