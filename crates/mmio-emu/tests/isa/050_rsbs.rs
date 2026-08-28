//! Decoding test for `rsbs`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_rsbs() {
    let inst = decode16(0x4248);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "rsbs decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Rsb"),
        "expected Rsb in {debug}"
    );
}
