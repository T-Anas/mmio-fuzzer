//! Execution test for `rev`.

use super::harness::run_with;

#[test]
fn exec_rev() {
    let mut core = run_with(
        &[0x08, 0xba],
        |cpu| {
            cpu.r[1] = 0x11223344;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x44332211, "r0");
}
