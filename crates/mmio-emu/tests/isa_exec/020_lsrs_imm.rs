//! Execution test for `lsrs_imm`.

use super::harness::run_with;

#[test]
fn exec_lsrs_imm() {
    let mut core = run_with(
        &[0x48, 0x08],
        |cpu| {
            cpu.r[1] = 0x00000008;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000004, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
}
