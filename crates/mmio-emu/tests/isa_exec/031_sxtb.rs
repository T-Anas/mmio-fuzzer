//! Execution test for `sxtb`.

use super::harness::run_with;

#[test]
fn exec_sxtb() {
    let mut core = run_with(&[0x48, 0xb2], |cpu| { cpu.r[1] = 0x00000080; }, 1);
    assert_eq!(core.cpu.r[0], 0xffffff80, "r0");
}
