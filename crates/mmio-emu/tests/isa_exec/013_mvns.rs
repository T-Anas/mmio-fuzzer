//! Execution test for `mvns`.

use super::harness::run_with;

#[test]
fn exec_mvns() {
    let mut core = run_with(
        &[0xc8, 0x43],
        |cpu| {
            cpu.r[1] = 0x00000000;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xffffffff, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), true, "flag n");
}
