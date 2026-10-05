//! Execution test for `orrs`.

use super::harness::run_with;

#[test]
fn exec_orrs() {
    let core = run_with(
        &[0x08, 0x43],
        |cpu| {
            cpu.r[0] = 0x000000f0;
            cpu.r[1] = 0x0000000f;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x000000ff, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
