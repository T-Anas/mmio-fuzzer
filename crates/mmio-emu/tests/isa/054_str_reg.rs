//! Decoding test for `str_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_str_reg() {
    let inst = mmio_emu::decode::decode16(0x5088);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "str_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
