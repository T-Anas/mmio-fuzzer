//! Execution test for `rev16`.

use super::harness::run_with;

#[test]
fn exec_rev16() {
    let core = run_with(
        &[0x48, 0xba],
        |cpu| {
            cpu.r[1] = 0x11223344;
        },
        1,
    );
    assert_eq!(core.cpu.r[0], 0x22114433, "r0");
}
