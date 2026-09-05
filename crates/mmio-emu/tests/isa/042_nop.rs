//! Decoding test for `nop`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_nop() {
    let inst = mmio_emu::decode::decode16(0xBF00);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "nop decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Nop"), "expected Nop in {debug}");
}
