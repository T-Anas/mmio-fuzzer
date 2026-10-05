//! Execution test for `rors`.

use super::harness::run_with;

#[test]
fn exec_rors() {
    let core = run_with(
        &[0xc8, 0x41],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000001;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x80000000, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.n(), "flag n");
    assert!(core.cpu.xpsr.c(), "flag c");
}
