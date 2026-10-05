//! Execution test for `bics`.

use super::harness::run_with;

#[test]
fn exec_bics() {
    let core = run_with(
        &[0x88, 0x43],
        |cpu| {
            cpu.r[0] = 0x000000ff;
            cpu.r[1] = 0x0000000f;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x000000f0, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
