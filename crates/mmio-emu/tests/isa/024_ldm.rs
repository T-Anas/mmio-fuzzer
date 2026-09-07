//! Decoding test for `ldm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_ldm() {
    let inst = mmio_emu::decode::decode16(0xc806);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "ldm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("LoadStoreMulti"),
        "expected LoadStoreMulti in {debug}"
    );
}
