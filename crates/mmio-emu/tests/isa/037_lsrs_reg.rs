//! Decoding test for `lsrs_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_lsrs_reg() {
    let inst = mmio_emu::decode::decode16(0x40c8);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "lsrs_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Lsr"), "expected Lsr in {debug}");
}
