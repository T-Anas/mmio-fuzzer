//! Decoding test for `bl`.
//!
//! Encoding produced by `arm-none-eabi-as -mcpu=cortex-m0`.

use mmio_emu::decode::Inst;

#[test]
fn decodes_bl() {
    let inst = mmio_emu::decode::decode32(0xf7ff, 0xfffe);
    assert!(
        !matches!(inst, Inst::Unsupported(_)),
        "bl decoded as Unsupported: {inst:?}"
    );
    let debug = format!("{inst:?}");
    assert!(
        debug.contains("BranchLink"),
        "expected BranchLink in {debug}"
    );
}
