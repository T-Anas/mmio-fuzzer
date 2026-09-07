//! Decoding test for `tst`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_tst() {
    let inst = mmio_emu::decode::decode16(0x4208);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "tst decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Tst"), "expected Tst in {debug}");
}
