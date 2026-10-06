//! Guard-hole detection.
//!
//! The `guard_demo` fixture reads one byte past its input buffer. With a tight
//! memory layout that leaves a hole after the buffer, the out-of-bounds read
//! lands in unmapped memory and is reported as a `GuardHit` — the signal a
//! parser bug would produce.

use mmio_fuzz::{AnomalyKind, Engine, EngineConfig, Firmware, Input, MemoryLayout};

const GUARD_ELF: &[u8] = include_bytes!("../../../fixtures/prebuilt/guard_demo.elf");

fn engine() -> Engine {
    let firmware = Firmware::from_elf_bytes(GUARD_ELF).expect("parse guard_demo");
    let layout = MemoryLayout::tight()
        .with_stack(0x2000_0000, 0x1000)
        .with_input(0x2000_4000, 0x1000)
        .with_guard(0x2000_5000, 0x2000);
    let config = EngineConfig {
        layout,
        max_steps: 1_000,
        no_edge_limit: 200,
        ..EngineConfig::default()
    };
    Engine::new(firmware, config)
}

#[test]
fn out_of_bounds_read_hits_the_guard() {
    let engine = engine();
    let (anomaly, exit) = engine.check_outcome(&Input::from_vec(vec![0u8; 4]));
    assert!(
        exit.is_none(),
        "the fixture should not reach the halt register"
    );
    let anomaly = anomaly.expect("the OOB read should fault");
    assert_eq!(anomaly.kind, AnomalyKind::GuardHit, "{anomaly:?}");
}

#[test]
fn out_of_bounds_read_hits_a_redzone_inside_ram() {
    // The fixture reads at 0x2000_6000. Here that address is mapped but
    // poisoned, so it is an `OutOfBounds` rather than an unmapped guard hit.
    let firmware = Firmware::from_elf_bytes(GUARD_ELF).expect("parse guard_demo");
    let layout = MemoryLayout::tight()
        .with_stack(0x2000_0000, 0x1000)
        .with_input(0x2000_4000, 0x4000)
        .with_redzone(0x2000_5000, 0x2000);
    let config = EngineConfig {
        layout,
        max_steps: 1_000,
        no_edge_limit: 200,
        ..EngineConfig::default()
    };
    let engine = Engine::new(firmware, config);
    let (anomaly, _exit) = engine.check_outcome(&Input::from_vec(vec![0u8; 4]));
    let anomaly = anomaly.expect("the redzone read should fault");
    assert_eq!(anomaly.kind, AnomalyKind::OutOfBounds, "{anomaly:?}");
}
