//! Decoding test for `lsrs_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_lsrs_imm() {
    let inst = mmio_emu::decode::decode16(0x08c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "lsrs_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Lsr"), "expected Lsr in {debug}");
}
