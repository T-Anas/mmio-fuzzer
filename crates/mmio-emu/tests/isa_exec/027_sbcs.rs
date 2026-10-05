//! Execution test for `sbcs`.

use super::harness::run_with;

#[test]
fn exec_sbcs() {
    let core = run_with(
        &[0x88, 0x41],
        |cpu| {
            cpu.r[0] = 0x00000005;
            cpu.r[1] = 0x00000002;
            cpu.xpsr.set_c(true);
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000003, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.c(), "flag c");
}
