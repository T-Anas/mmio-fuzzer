//! Decoding test for `mov_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_mov_reg() {
    let inst = mmio_emu::decode::decode16(0x4640);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "mov_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Mov"), "expected Mov in {debug}");
}
