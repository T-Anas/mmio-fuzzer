//! Decoding test for `cmn`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_cmn() {
    let inst = mmio_emu::decode::decode16(0x42c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cmn decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Cmn"), "expected Cmn in {debug}");
}
