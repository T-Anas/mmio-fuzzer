//! Decoding test for `lsls_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_lsls_imm() {
    let inst = mmio_emu::decode::decode16(0x00c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "lsls_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Lsl"), "expected Lsl in {debug}");
}
