//! Execution test for `sub_sp_imm`.

use super::harness::run_with;

#[test]
fn exec_sub_sp_imm() {
    let core = run_with(&[0x84, 0xb0], |_cpu| {}, 1);
    assert_eq!(core.cpu.sp(), 0x200007f0, "sp");
}
