//! Execution test for `cmp_reg`.

use super::harness::run_with;

#[test]
fn exec_cmp_reg() {
    let core = run_with(
        &[0x88, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000004;
            cpu.r[1] = 0x00000005;
        },
        1,
    );
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.c(), "flag c");
    assert!(core.cpu.xpsr.n(), "flag n");
}
