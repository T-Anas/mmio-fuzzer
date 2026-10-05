//! Execution test for `uxth`.

use super::harness::run_with;

#[test]
fn exec_uxth() {
    let mut core = run_with(
        &[0x88, 0xb2],
        |cpu| {
            cpu.r[1] = 0x12345678;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00005678, "r0");
}
