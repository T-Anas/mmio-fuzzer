//! End-to-end acceptance test.
//!
//! This is the claim the project makes, made falsifiable: given a real
//! Cortex-M0 firmware that polls a status register and later branches on an
//! MMIO-provided command, the engine must (a) infer the polled status
//! register, (b) unblock it, and (c) discover the seeded memory-safety bug.

use mmio_fuzz::{Engine, EngineConfig, Firmware};

const DEMO_ELF: &[u8] = include_bytes!("../../../fixtures/prebuilt/uart_demo.elf");

#[test]
fn finds_seeded_bug_and_infers_ready_bit() {
    let firmware = Firmware::from_elf_bytes(DEMO_ELF).expect("parse fixture");
    let config = EngineConfig {
        max_steps: 5_000,
        iterations: 2_000,
        no_edge_limit: 512,
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(firmware, config);
    let stats = engine.run();

    eprintln!("statistics: {stats:?}");
    eprintln!("{}", engine.model().report());
    for finding in engine.findings() {
        eprintln!("finding: {:?} at {:#010x}", finding.finding.kind, finding.finding.pc);
    }

    // The firmware polls UART_STATUS at 0x4000_0004 until bit 0 reads set.
    let status = engine
        .model()
        .register(0x4000_0004)
        .expect("status register should have been inferred");
    assert!(status.polled, "status register should be detected as polled");
    assert_eq!(
        status.ready_value,
        Some(1),
        "the ready bit should be inferred as bit 0"
    );

    // The seeded bug is reachable for any command with the top bit set.
    assert!(
        !engine.findings().is_empty(),
        "engine should report the seeded bug"
    );
    assert!(
        engine
            .findings()
            .iter()
            .any(|finding| finding.finding.kind.is_target_bug()),
        "at least one finding should be a target bug"
    );
}
