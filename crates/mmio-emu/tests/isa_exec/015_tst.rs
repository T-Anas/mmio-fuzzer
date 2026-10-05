//! Execution test for `tst`.

use super::harness::run_with;

#[test]
fn exec_tst() {
    let core = run_with(
        &[0x08, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000000;
            cpu.r[1] = 0x00000008;
        },
        1,
    );
    assert!(core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
