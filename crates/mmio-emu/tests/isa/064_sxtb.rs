//! Decoding test for `sxtb`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_sxtb() {
    let inst = decode16(0xb248);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "sxtb decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Sxtb"),
        "expected Sxtb in {debug}"
    );
}
