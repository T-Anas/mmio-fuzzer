//! `mmio-fuzz`: a command-line front end for the MMIO-aware fuzzer.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use mmio_fuzz::{Engine, EngineConfig, Firmware, Input, MemoryLayout, Testcase};
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
#[allow(clippy::large_enum_variant)] // the run subcommand legitimately has many flags
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
        /// Load the fuzz input into RAM at `BASE:SIZE`, e.g. `0x20004000:0x1000`.
        #[arg(long)]
        input_mem: Option<String>,
        /// Deliver the fuzz input through a UART-like device at this base.
        #[arg(long)]
        stream_uart: Option<String>,
        /// Halt register address; writing it ends a run cleanly.
        #[arg(long)]
        halt_addr: Option<String>,
        /// Replace the default RAM window with `BASE:SIZE`.
        #[arg(long)]
        ram: Option<String>,
        /// Map a stack region `BASE:SIZE`.
        #[arg(long)]
        stack: Option<String>,
        /// Map a heap region `BASE:SIZE`.
        #[arg(long)]
        heap: Option<String>,
        /// Leave `BASE:SIZE` unmapped, reported as a guard hit (repeatable).
        #[arg(long = "guard")]
        guards: Vec<String>,
        /// Seed corpus with the bytes of this file.
        #[arg(long)]
        seed_file: Option<PathBuf>,
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
            input_mem,
            stream_uart,
            halt_addr,
            ram,
            stack,
            heap,
            guards,
            seed_file,
        } => run(
            firmware,
            iterations,
            seed,
            max_steps,
            out,
            LayoutArgs {
                input_mem,
                stream_uart,
                halt_addr,
                ram,
                stack,
                heap,
                guards,
                seed_file,
            },
        ),
        Command::Infer { firmware, out } => infer(firmware, out),
        Command::Replay { case, firmware } => replay(case, firmware),
        Command::Inspect { model } => inspect(model),
    }
}

/// Layout-related command-line arguments.
#[derive(Default)]
struct LayoutArgs {
    input_mem: Option<String>,
    stream_uart: Option<String>,
    halt_addr: Option<String>,
    ram: Option<String>,
    stack: Option<String>,
    heap: Option<String>,
    guards: Vec<String>,
    seed_file: Option<PathBuf>,
}

fn parse_u32(text: &str) -> Result<u32> {
    let trimmed = text.trim();
    let value = if let Some(hex) = trimmed.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
    } else {
        trimmed.parse()
    };
    value.with_context(|| format!("not a u32: {text:?}"))
}

fn parse_range(text: &str) -> Result<(u32, u32)> {
    let (base, size) = text
        .split_once(':')
        .with_context(|| format!("expected BASE:SIZE, got {text:?}"))?;
    Ok((parse_u32(base)?, parse_u32(size)?))
}

fn build_config(
    seed: Option<u64>,
    max_steps: Option<u64>,
    iterations: u64,
    args: &LayoutArgs,
) -> Result<EngineConfig> {
    let defaults = EngineConfig::default();
    let mut layout = if args.ram.is_some() || !args.guards.is_empty() || args.stack.is_some() {
        MemoryLayout::tight()
    } else {
        defaults.layout.clone()
    };
    if let Some(range) = &args.ram {
        layout.ram = Some(parse_range(range)?);
    }
    if let Some(range) = &args.stack {
        layout.stack = Some(parse_range(range)?);
    }
    if let Some(range) = &args.heap {
        layout.heap = Some(parse_range(range)?);
    }
    if let Some(range) = &args.input_mem {
        layout.input = Some(parse_range(range)?);
    }
    for guard in &args.guards {
        let (base, size) = parse_range(guard)?;
        layout.guards.push((base, size));
    }

    let stream_uart = args.stream_uart.as_deref().map(parse_u32).transpose()?;
    let halt_addr = args.halt_addr.as_deref().map(parse_u32).transpose()?;

    Ok(EngineConfig {
        iterations,
        seed: seed.unwrap_or(defaults.seed),
        max_steps: max_steps.unwrap_or(defaults.max_steps),
        layout,
        stream_uart,
        halt_addr,
        ..defaults
    })
}

fn load_firmware(path: &PathBuf) -> Result<Firmware> {
    Firmware::from_file(path).with_context(|| format!("loading firmware {}", path.display()))
}

fn seed_engine(engine: &mut Engine, seed_file: &Option<PathBuf>) -> Result<()> {
    if let Some(path) = seed_file {
        let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        engine.seed_corpus(Input::from_vec(bytes));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run(
    firmware: PathBuf,
    iterations: u64,
    seed: Option<u64>,
    max_steps: Option<u64>,
    out: PathBuf,
    args: LayoutArgs,
) -> Result<()> {
    let firmware = load_firmware(&firmware)?;
    println!("firmware {} ({:?})", firmware.hash_hex(), firmware.entry());

    let config = build_config(seed, max_steps, iterations, &args)?;
    let mut engine = Engine::new(firmware, config);
    seed_engine(&mut engine, &args.seed_file)?;
    let stats = engine.run();

    println!("\n== inferred hardware model ==");
    print!("{}", engine.model().report());
    println!(
        "\n== summary ==\nexecutions: {}\nedges: {}\ncorpus: {}\nregisters: {}\nfindings: {}\nclean exits: {}\nelapsed: {:.2}s",
        stats.executions,
        stats.edges,
        stats.corpus,
        stats.discovered_registers,
        stats.findings,
        stats.clean_exits,
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

    let mut config = EngineConfig::default();
    if let Some(repro) = &testcase.repro {
        config.layout = repro.layout.clone();
        config.stream_uart = repro.stream_uart;
        config.halt_addr = repro.halt_addr;
    }
    let mut engine = Engine::new(firmware, config);
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
                bail!(
                    "anomaly kind {:?} differs from recorded {:?}",
                    anomaly.kind,
                    testcase.finding.kind
                )
            }
        }
        None => bail!("finding did not reproduce"),
    }
}

fn inspect(model: PathBuf) -> Result<()> {
    let text =
        std::fs::read_to_string(&model).with_context(|| format!("reading {}", model.display()))?;
    let model = HardwareModel::from_json(&text).context("parsing model")?;
    print!("{}", model.report());
    Ok(())
}
