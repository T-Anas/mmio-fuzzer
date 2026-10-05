//! Execution test for `mvns`.

use super::harness::run_with;

#[test]
fn exec_mvns() {
    let core = run_with(
        &[0xc8, 0x43],
        |cpu| {
            cpu.r[1] = 0x00000000;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xffffffff, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.n(), "flag n");
}
