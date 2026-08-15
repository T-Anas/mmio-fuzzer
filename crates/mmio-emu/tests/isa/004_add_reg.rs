//! Decoding test for `add_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_add_reg() {
    let inst = decode16(0x1888);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "add_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("AddSub"),
        "expected AddSub in {debug}"
    );
}
