//! Execution test for `adds_imm8`.

use super::harness::run_with;

#[test]
fn exec_adds_imm8() {
    let core = run_with(
        &[0x03, 0x30],
        |cpu| {
            cpu.r[0] = 0x00000005;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000008, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.c(), "flag c");
}
