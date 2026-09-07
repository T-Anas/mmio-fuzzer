//! Decoding test for `cpsid`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_cpsid() {
    let inst = mmio_emu::decode::decode16(0xb672);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cpsid decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Cps"), "expected Cps in {debug}");
}
