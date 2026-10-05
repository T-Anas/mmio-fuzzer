//! Execution test for `cmn`.

use super::harness::run_with;

#[test]
fn exec_cmn() {
    let core = run_with(
        &[0xc8, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000001;
        },
        1,
    );
    assert!(!core.cpu.xpsr.z(), "flag z");
    assert!(!core.cpu.xpsr.c(), "flag c");
    assert!(!core.cpu.xpsr.n(), "flag n");
}
