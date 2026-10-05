//! Execution test for `muls`.

use super::harness::run_with;

#[test]
fn exec_muls() {
    let core = run_with(
        &[0x48, 0x43],
        |cpu| {
            cpu.r[0] = 0x00000006;
            cpu.r[1] = 0x00000007;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x0000002a, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
