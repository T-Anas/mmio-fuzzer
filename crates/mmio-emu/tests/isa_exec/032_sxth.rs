//! Execution test for `sxth`.

use super::harness::run_with;

#[test]
fn exec_sxth() {
    let mut core = run_with(&[0x08, 0xb2], |cpu| { cpu.r[1] = 0x00008000; }, 1);
    assert_eq!(core.cpu.r[0], 0xffff8000, "r0");
}
