//! Execution test for `movs_imm`.

use super::harness::run_with;

#[test]
fn exec_movs_imm() {
    let mut core = run_with(&[0x05, 0x20], |_cpu| {}, 1);
    assert_eq!(core.cpu.r[0], 0x00000005, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
