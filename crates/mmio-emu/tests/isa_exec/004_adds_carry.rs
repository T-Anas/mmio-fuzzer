//! Execution test for `adds_carry`.

use super::harness::run_with;

#[test]
fn exec_adds_carry() {
    let mut core = run_with(
        &[0x88, 0x18],
        |cpu| {
            cpu.r[1] = 0xffffffff;
            cpu.r[2] = 0x00000001;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000000, "r0");
    assert_eq!(core.cpu.xpsr.z(), true, "flag z");
    assert_eq!(core.cpu.xpsr.c(), true, "flag c");
    assert_eq!(core.cpu.xpsr.v(), false, "flag v");
}
