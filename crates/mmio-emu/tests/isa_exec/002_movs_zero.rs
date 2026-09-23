//! Execution test for `movs_zero`.

use super::harness::run_with;

#[test]
fn exec_movs_zero() {
    let mut core = run_with(&[0x00, 0x20], |_cpu| {}, 1);
    assert_eq!(core.cpu.r[0], 0x00000000, "r0");
    assert_eq!(core.cpu.xpsr.z(), true, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
