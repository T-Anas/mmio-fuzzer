//! Execution test for `ands`.

use super::harness::run_with;

#[test]
fn exec_ands() {
    let mut core = run_with(
        &[0x08, 0x40],
        |cpu| {
            cpu.r[0] = 0x000000f0;
            cpu.r[1] = 0x0000003c;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000030, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
