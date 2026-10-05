//! Execution test for `subs_imm8`.

use super::harness::run_with;

#[test]
fn exec_subs_imm8() {
    let mut core = run_with(
        &[0x03, 0x38],
        |cpu| {
            cpu.r[0] = 0x00000005;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000002, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), true, "flag c");
}
