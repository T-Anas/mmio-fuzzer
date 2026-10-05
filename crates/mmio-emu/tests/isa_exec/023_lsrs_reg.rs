//! Execution test for `lsrs_reg`.

use super::harness::run_with;

#[test]
fn exec_lsrs_reg() {
    let mut core = run_with(
        &[0xc8, 0x40],
        |cpu| {
            cpu.r[0] = 0x00000010;
            cpu.r[1] = 0x00000002;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000004, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
}
