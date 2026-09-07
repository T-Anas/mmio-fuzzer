//! Decoding test for `rev16`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_rev16() {
    let inst = mmio_emu::decode::decode16(0xba48);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "rev16 decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Rev"), "expected Rev in {debug}");
}
