//! Execution test for `subs_borrow`.

use super::harness::run_with;

#[test]
fn exec_subs_borrow() {
    let core = run_with(
        &[0x88, 0x1a],
        |cpu| {
            cpu.r[1] = 0x00000003;
            cpu.r[2] = 0x0000000a;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xfffffff9, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.c(), "flag c");
    assert!(core.cpu.xpsr.n(), "flag n");
}
