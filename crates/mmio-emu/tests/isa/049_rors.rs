//! Decoding test for `rors`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_rors() {
    let inst = mmio_emu::decode::decode16(0x41c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "rors decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Ror"), "expected Ror in {debug}");
}
