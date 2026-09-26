//! Execution test for `eors`.

use super::harness::run_with;

#[test]
fn exec_eors() {
    let mut core = run_with(&[0x48, 0x40], |cpu| { cpu.r[0] = 0x000000ff; cpu.r[1] = 0x0000000f; }, 1);
    assert_eq!(core.cpu.r[0], 0x000000f0, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
