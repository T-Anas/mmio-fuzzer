//! Decoding test for `ldrsh`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_ldrsh() {
    let inst = mmio_emu::decode::decode16(0x5e88);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldrsh decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
