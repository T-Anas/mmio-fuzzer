//! Decoding test for `cmp_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_cmp_reg() {
    let inst = decode16(0x4288);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "cmp_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Cmp"),
        "expected Cmp in {debug}"
    );
}
