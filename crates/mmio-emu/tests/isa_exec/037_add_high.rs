//! Execution test for `add_high`.

use super::harness::run_with;

#[test]
fn exec_add_high() {
    let mut core = run_with(
        &[0x40, 0x44],
        |cpu| {
            cpu.r[0] = 0x00000001;
            cpu.r[8] = 0x00000002;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000003, "r0");
}
