//! Execution test for `movs_zero`.

use super::harness::run_with;

#[test]
fn exec_movs_zero() {
    let core = run_with(&[0x00, 0x20], |_cpu| {}, 1);
    assert_eq!(core.cpu.r[0], 0x00000000, "r0");
    assert!(core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
