//! Execution test for `asrs_imm`.

use super::harness::run_with;

#[test]
fn exec_asrs_imm() {
    let core = run_with(
        &[0x48, 0x10],
        |cpu| {
            cpu.r[1] = 0xfffffffe;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xffffffff, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.n(), "flag n");
    assert!(!core.cpu.xpsr.c(), "flag c");
}
