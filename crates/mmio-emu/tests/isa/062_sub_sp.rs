//! Decoding test for `sub_sp`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_sub_sp() {
    let inst = decode16(0xb084);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sub_sp decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("SubSp"),
        "expected SubSp in {debug}"
    );
}
