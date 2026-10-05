//! Execution test for `add_sp_imm`.

use super::harness::run_with;

#[test]
fn exec_add_sp_imm() {
    let core = run_with(&[0x04, 0xb0], |_cpu| {}, 1);
    assert_eq!(core.cpu.sp(), 0x20000810, "sp");
}
