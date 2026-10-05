//! Execution test for `cmp_imm`.

use super::harness::run_with;

#[test]
fn exec_cmp_imm() {
    let mut core = run_with(
        &[0x05, 0x28],
        |cpu| {
            cpu.r[0] = 0x00000005;
        },
        1,
    );
    assert_eq!(core.cpu.xpsr.z(), true, "flag z");
    assert_eq!(core.cpu.xpsr.c(), true, "flag c");
}
