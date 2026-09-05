//! Decoding test for `str_imm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_str_imm() {
    let inst = mmio_emu::decode::decode16(0x6048);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "str_imm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
