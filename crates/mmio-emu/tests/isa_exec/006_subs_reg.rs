//! Execution test for `subs_reg`.

use super::harness::run_with;

#[test]
fn exec_subs_reg() {
    let mut core = run_with(
        &[0x88, 0x1a],
        |cpu| {
            cpu.r[1] = 0x0000000a;
            cpu.r[2] = 0x00000003;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000007, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), true, "flag c");
    assert_eq!(core.cpu.xpsr.v(), false, "flag v");
}
