//! Decoding test for `add_sp`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_add_sp() {
    let inst = mmio_emu::decode::decode16(0xa802);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "add_sp decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Adr"), "expected Adr in {debug}");
}
