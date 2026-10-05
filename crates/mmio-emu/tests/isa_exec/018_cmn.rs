//! Execution test for `cmn`.

use super::harness::run_with;

#[test]
fn exec_cmn() {
    let mut core = run_with(
        &[0xc8, 0x42],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[1] = 0x00000001;
        },
        1,
    );
    assert_eq!(core.cpu.xpsr.z(), false, "flag z");
    assert_eq!(core.cpu.xpsr.c(), false, "flag c");
    assert_eq!(core.cpu.xpsr.n(), false, "flag n");
}
