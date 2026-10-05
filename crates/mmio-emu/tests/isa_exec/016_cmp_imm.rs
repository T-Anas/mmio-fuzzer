//! Execution test for `cmp_imm`.

use super::harness::run_with;

#[test]
fn exec_cmp_imm() {
    let core = run_with(
        &[0x05, 0x28],
        |cpu| {
            cpu.r[0] = 0x00000005;
        },
        1,
    );
    assert!(core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.c(), "flag c");
}
