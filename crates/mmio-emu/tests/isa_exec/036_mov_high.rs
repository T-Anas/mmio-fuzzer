//! Execution test for `mov_high`.

use super::harness::run_with;

#[test]
fn exec_mov_high() {
    let mut core = run_with(
        &[0x40, 0x46],
        |cpu| {
            cpu.r[8] = 0x00000055;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000055, "r0");
}
