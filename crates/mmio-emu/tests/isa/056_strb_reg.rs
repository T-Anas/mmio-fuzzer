//! Decoding test for `strb_reg`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_strb_reg() {
    let inst = mmio_emu::decode::decode16(0x5488);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "strb_reg decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(debug.contains("LoadStore"), "expected LoadStore in {debug}");
}
