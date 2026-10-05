//! Execution test for `uxtb`.

use super::harness::run_with;

#[test]
fn exec_uxtb() {
    let mut core = run_with(
        &[0xc8, 0xb2],
        |cpu| {
            cpu.r[1] = 0x00001234;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x00000034, "r0");
}
