//! Execution test for `revsh`.

use super::harness::run_with;

#[test]
fn exec_revsh() {
    let core = run_with(
        &[0xc8, 0xba],
        |cpu| {
            cpu.r[1] = 0x00001180;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0xffff8011, "r0");
}
