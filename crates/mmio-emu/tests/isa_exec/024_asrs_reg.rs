//! Execution test for `asrs_reg`.

use super::harness::run_with;

#[test]
fn exec_asrs_reg() {
    let core = run_with(
        &[0x08, 0x41],
        |cpu| {
            cpu.r[0] = 0xffff0000;
            cpu.r[1] = 0x00000004;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xfffff000, "r0");
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(core.cpu.xpsr.n(), "flag n");
}
