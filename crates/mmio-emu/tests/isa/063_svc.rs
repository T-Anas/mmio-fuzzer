//! Decoding test for `svc`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_svc() {
    let inst = mmio_emu::decode::decode16(0xdf00);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "svc decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("Svc"), "expected Svc in {debug}");
}
