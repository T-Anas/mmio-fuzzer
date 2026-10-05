//! Execution test for `adcs`.

use super::harness::run_with;

#[test]
fn exec_adcs() {
    let mut core = run_with(
        &[0x48, 0x41],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000001;
            cpu.xpsr.set_c(true);
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000003, "r0");
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
}
