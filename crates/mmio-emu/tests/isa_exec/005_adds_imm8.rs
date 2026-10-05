//! Execution test for `adds_imm8`.

use super::harness::run_with;

#[test]
fn exec_adds_imm8() {
    let mut core = run_with(
        &[0x03, 0x30],
        |cpu| {
            cpu.r[0] = 0x00000005;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000008, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
}
