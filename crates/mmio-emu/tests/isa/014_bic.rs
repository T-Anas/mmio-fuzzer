//! Decoding test for `bic`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::{decode16, decode32, Inst};

#[test]
fn decodes_bic() {
    let inst = decode16(0x4388);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "bic decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("Bic"),
        "expected Bic in {debug}"
    );
}
