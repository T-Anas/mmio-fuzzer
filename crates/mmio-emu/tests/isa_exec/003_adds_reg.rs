//! Execution test for `adds_reg`.

use super::harness::run_with;

#[test]
fn exec_adds_reg() {
    let mut core = run_with(
        &[0x88, 0x18],
        |cpu| {
            cpu.r[1] = 0x00000003;
            cpu.r[2] = 0x00000004;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000007, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
    assert_eq!(core.cpu.xpsr.v(), false, "flag v");
}
