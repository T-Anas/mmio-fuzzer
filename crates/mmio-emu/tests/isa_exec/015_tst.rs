//! Execution test for `tst`.

use super::harness::run_with;

#[test]
fn exec_tst() {
    let mut core = run_with(
        &[0x08, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000000;
            cpu.r[1] = 0x00000008;
        },
        1,
    );
    assert_eq!(core.cpu.xpsr.z(), true, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
