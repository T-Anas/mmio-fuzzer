//! Execution test for `lsls_reg`.

use super::harness::run_with;

#[test]
fn exec_lsls_reg() {
    let core = run_with(
        &[0x88, 0x40],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000004;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000010, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
}
