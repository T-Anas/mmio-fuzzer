//! Decoding test for `ldrsb`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_ldrsb() {
    let inst = decode16(0x5688);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldrsb decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("LoadStore"),
        "expected LoadStore in {debug}"
    );
}
