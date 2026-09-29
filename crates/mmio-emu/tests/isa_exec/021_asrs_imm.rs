//! Execution test for `asrs_imm`.

use super::harness::run_with;

#[test]
fn exec_asrs_imm() {
    let mut core = run_with(&[0x48, 0x10], |cpu| { cpu.r[1] = 0xfffffffe; }, 1);
    assert_eq!(core.cpu.r[0], 0xffffffff, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), true, "flag n");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
}
