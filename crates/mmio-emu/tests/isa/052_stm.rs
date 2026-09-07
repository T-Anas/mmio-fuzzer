//! Decoding test for `stm`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_stm() {
    let inst = mmio_emu::decode::decode16(0xc006);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "stm decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("LoadStoreMulti"),
        "expected LoadStoreMulti in {debug}"
    );
}
