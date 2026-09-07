//! Decoding test for `adcs`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_adcs() {
    let inst = mmio_emu::decode::decode16(0x4148);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "adcs decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Adc"), "expected Adc in {debug}");
}
