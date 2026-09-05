//! Decoding test for `mvns`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_mvns() {
    let inst = mmio_emu::decode::decode16(0x43c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "mvns decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Mvn"), "expected Mvn in {debug}");
}
