//! Run the engine over the bundled `uart_demo` fixture and print what it
//! learns. Useful as a smoke test and as a template for embedding the engine.
//!
//! ```sh
//! cargo run --example run_uart_demo -p mmio-fuzz
//! ```

use std::path::Path;

use mmio_fuzz::{Engine, EngineConfig, Firmware};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/prebuilt/uart_demo.elf");
    let firmware = Firmware::from_file(path)?;
    println!(
        "firmware {} at {:#010x}",
        firmware.hash_hex(),
        firmware.entry()
    );

    let config = EngineConfig {
        iterations: 2_000,
        max_steps: 5_000,
        no_edge_limit: 512,
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(firmware, config);
    let stats = engine.run();

    print!("{}", engine.model().report());
    println!(
        "executions={} edges={} corpus={} findings={} elapsed={:.2}s",
        stats.executions,
        stats.edges,
        stats.corpus,
        stats.findings,
        stats.elapsed.as_secs_f64()
    );
    for finding in engine.findings() {
        println!(
            "  {:?} at {:#010x}: {}",
            finding.finding.kind, finding.finding.pc, finding.finding.detail
        );
    }
    Ok(())
}
