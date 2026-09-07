//! Decoding test for `ldrb_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_ldrb_reg() {
    let inst = mmio_emu::decode::decode16(0x5c88);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldrb_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
