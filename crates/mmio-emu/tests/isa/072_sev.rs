//! Decoding test for `sev`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_sev() {
    let inst = mmio_emu::decode::decode16(0xbf40);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sev decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Nop"), "expected Nop in {debug}");
}
