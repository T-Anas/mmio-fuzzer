//! Decoding test for `asrs_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_asrs_imm() {
    let inst = mmio_emu::decode::decode16(0x1088);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "asrs_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Asr"), "expected Asr in {debug}");
}
