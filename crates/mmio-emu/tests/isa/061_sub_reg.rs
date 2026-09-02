//! Decoding test for `sub_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_sub_reg() {
    let inst = decode16(0x1a88);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sub_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("AddSub"),
        "expected AddSub in {debug}"
    );
}
