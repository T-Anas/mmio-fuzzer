//! `mmio-fuzz`: a command-line front end for the MMIO-aware fuzzer.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use mmio_fuzz::{Engine, EngineConfig, Firmware, Input, Testcase};
use mmio_infer::HardwareModel;

#[derive(Parser)]
#[command(
    name = "mmio-fuzz",
    version,
    about = "MMIO-aware coverage-guided fuzzer for bare-metal Cortex-M firmware"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Profile a firmware, then fuzz it and report findings.
    Run {
        /// Firmware ELF image.
        firmware: PathBuf,
        /// Number of fuzzing iterations after profiling.
        #[arg(long, default_value_t = 10_000)]
        iterations: u64,
        /// RNG seed for reproducibility.
        #[arg(long)]
        seed: Option<u64>,
        /// Instruction budget per execution.
        #[arg(long)]
        max_steps: Option<u64>,
        /// Directory for testcases and the inferred model.
        #[arg(long, default_value = ".")]
        out: PathBuf,
    },
    /// Profile a firmware and print the inferred register model.
    Infer {
        firmware: PathBuf,
        /// Also write the model as JSON to this file.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Replay a saved testcase against its firmware.
    Replay {
        /// Testcase file (`.mmf`).
        case: PathBuf,
        /// Firmware ELF image.
        firmware: PathBuf,
    },
    /// Print a previously saved model.
    Inspect {
        /// Model JSON file.
        model: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run {
            firmware,
            iterations,
            seed,
            max_steps,
            out,
        } => run(firmware, iterations, seed, max_steps, out),
        Command::Infer { firmware, out } => infer(firmware, out),
        Command::Replay { case, firmware } => replay(case, firmware),
        Command::Inspect { model } => inspect(model),
    }
}

fn build_config(seed: Option<u64>, max_steps: Option<u64>, iterations: u64) -> EngineConfig {
    let defaults = EngineConfig::default();
    EngineConfig {
        iterations,
        seed: seed.unwrap_or(defaults.seed),
        max_steps: max_steps.unwrap_or(defaults.max_steps),
        ..defaults
    }
}

fn load_firmware(path: &PathBuf) -> Result<Firmware> {
    Firmware::from_file(path).with_context(|| format!("loading firmware {}", path.display()))
}

fn run(
    firmware: PathBuf,
    iterations: u64,
    seed: Option<u64>,
    max_steps: Option<u64>,
    out: PathBuf,
) -> Result<()> {
    let firmware = load_firmware(&firmware)?;
    println!("firmware {} ({:?})", firmware.hash_hex(), firmware.entry());

    let mut engine = Engine::new(firmware, build_config(seed, max_steps, iterations));
    let stats = engine.run();

    println!("\n== inferred hardware model ==");
    print!("{}", engine.model().report());
    println!(
        "\n== summary ==\nexecutions: {}\nedges: {}\ncorpus: {}\nregisters: {}\nfindings: {}\nelapsed: {:.2}s",
        stats.executions,
        stats.edges,
        stats.corpus,
        stats.discovered_registers,
        stats.findings,
        stats.elapsed.as_secs_f64()
    );

    std::fs::create_dir_all(&out).ok();
    let model_path = out.join("model.json");
    std::fs::write(&model_path, engine.model().to_json_pretty())
        .with_context(|| format!("writing {}", model_path.display()))?;
    println!("model written to {}", model_path.display());

    if engine.findings().is_empty() {
        println!("no findings");
    }
    for finding in engine.findings() {
        let path = out.join(finding.filename());
        std::fs::write(&path, finding.to_json())
            .with_context(|| format!("writing {}", path.display()))?;
        println!(
            "finding {:?} at {:#010x} -> {}",
            finding.finding.kind,
            finding.finding.pc,
            path.display()
        );
    }
    Ok(())
}

fn infer(firmware: PathBuf, out: Option<PathBuf>) -> Result<()> {
    let firmware = load_firmware(&firmware)?;
    let mut engine = Engine::new(firmware, EngineConfig::default());
    engine.profile();
    print!("{}", engine.model().report());
    if let Some(path) = out {
        std::fs::write(&path, engine.model().to_json_pretty())
            .with_context(|| format!("writing {}", path.display()))?;
        eprintln!("model written to {}", path.display());
    }
    Ok(())
}

fn replay(case: PathBuf, firmware: PathBuf) -> Result<()> {
    let text =
        std::fs::read_to_string(&case).with_context(|| format!("reading {}", case.display()))?;
    let testcase = Testcase::from_json(&text).context("parsing testcase")?;
    let firmware = load_firmware(&firmware)?;

    if firmware.hash_hex() != testcase.firmware_hash {
        eprintln!(
            "warning: firmware hash {} does not match testcase {}",
            firmware.hash_hex(),
            testcase.firmware_hash
        );
    }

    let mut engine = Engine::new(firmware, EngineConfig::default());
    engine.profile();
    let input = Input::from_vec(testcase.input.clone());
    match engine.check(&input) {
        Some(anomaly) => {
            println!(
                "reproduced {:?} at {:#010x}: {}",
                anomaly.kind, anomaly.pc, anomaly.detail
            );
            if anomaly.kind == testcase.finding.kind {
                println!("matches recorded finding");
                Ok(())
            } else {
                anyhow::bail!(
                    "anomaly kind {:?} differs from recorded {:?}",
                    anomaly.kind,
                    testcase.finding.kind
                )
            }
        }
        None => anyhow::bail!("finding did not reproduce"),
    }
}

fn inspect(model: PathBuf) -> Result<()> {
    let text =
        std::fs::read_to_string(&model).with_context(|| format!("reading {}", model.display()))?;
    let model = HardwareModel::from_json(&text).context("parsing model")?;
    print!("{}", model.report());
    Ok(())
}
