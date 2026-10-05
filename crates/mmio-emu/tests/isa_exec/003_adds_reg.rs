//! Execution test for `adds_reg`.

use super::harness::run_with;

#[test]
fn exec_adds_reg() {
    let core = run_with(
        &[0x88, 0x18],
        |cpu| {
            cpu.r[1] = 0x00000003;
            cpu.r[2] = 0x00000004;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000007, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.n(), "flag n");
    assert!(!core.cpu.xpsr.c(), "flag c");
    assert!(!core.cpu.xpsr.v(), "flag v");
}
