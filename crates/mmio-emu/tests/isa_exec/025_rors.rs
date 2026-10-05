//! Execution test for `rors`.

use super::harness::run_with;

#[test]
fn exec_rors() {
    let mut core = run_with(
        &[0xc8, 0x41],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000001;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x80000000, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), true, "flag n");
    assert_eq!(core.cpu.xpsr.c(), true, "flag c");
}
