//! Decoding test for `pop`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_pop() {
    let inst = mmio_emu::decode::decode16(0xbd02);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "pop decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Pop"), "expected Pop in {debug}");
}
