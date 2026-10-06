//! Interrupt injection: SysTick drives a handler that runs and returns.
//!
//! The `systick_demo` fixture enables SysTick and installs a handler that
//! counts ticks and halts after three. Reaching the halt register proves the
//! handler ran *and* that exception return restored the interrupted context.

use mmio_fuzz::{Engine, EngineConfig, Firmware, Input, MemoryLayout};

const SYSTICK_ELF: &[u8] = include_bytes!("../../../fixtures/prebuilt/systick_demo.elf");

#[test]
fn systick_handler_runs_three_times_then_halts() {
    let firmware = Firmware::from_elf_bytes(SYSTICK_ELF).expect("parse systick_demo");
    let layout = MemoryLayout::tight().with_stack(0x2000_0000, 0x1000);
    let config = EngineConfig {
        layout,
        halt_addr: Some(0x4000_f000),
        max_steps: 5_000,
        no_edge_limit: 4_000,
        ..EngineConfig::default()
    };
    let engine = Engine::new(firmware, config);
    let (anomaly, exit) = engine.check_outcome(&Input::new());
    assert!(anomaly.is_none(), "unexpected anomaly: {anomaly:?}");
    assert_eq!(
        exit,
        Some(0),
        "the SysTick handler should have halted the run"
    );
}
