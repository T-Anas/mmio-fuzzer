//! Execution test for `cmp_reg`.

use super::harness::run_with;

#[test]
fn exec_cmp_reg() {
    let mut core = run_with(
        &[0x88, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000004;
            cpu.r[1] = 0x00000005;
        },
        1,
    );
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
    assert_eq!(core.cpu.xpsr.n(), true, "flag n");
}
