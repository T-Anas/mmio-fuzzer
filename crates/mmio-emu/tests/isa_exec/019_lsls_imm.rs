//! Execution test for `lsls_imm`.

use super::harness::run_with;

#[test]
fn exec_lsls_imm() {
    let core = run_with(
        &[0xc8, 0x00],
        |cpu| {
            cpu.r[1] = 0x00000001;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000008, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.c(), "flag c");
}
