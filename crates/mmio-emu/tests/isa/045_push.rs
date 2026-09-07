//! Decoding test for `push`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_push() {
    let inst = mmio_emu::decode::decode16(0xb502);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "push decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Push"), "expected Push in {debug}");
}
