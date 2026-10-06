//! The fuzzing engine.
//!
//! Two phases:
//!
//! 1. **Profiling.** Run the firmware with a blank model to observe which
//!    peripherals it touches. Infer a first [`HardwareModel`], then, for every
//!    register the firmware polls, search for a value that lets it make
//!    progress. That search is the MMIO-aware step: it is how we learn that
//!    bit 0 of the status register means "ready".
//! 2. **Fuzzing.** Mutate inputs that bias peripheral responses and keep any
//!    input that reaches new coverage or triggers an anomaly.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use mmio_core::{Access, CoreError, FnObserver, PhysAddr};
use mmio_emu::{CortexM, Cpu};
use mmio_infer::{infer, AccessLog, HardwareModel, InferConfig, RegisterModel};

use crate::anomaly::{Anomaly, AnomalyKind};
use crate::coverage::Coverage;
use crate::input::Input;
use crate::machine::{self, Firmware, MemoryLayout};
use crate::mutate::Mutator;
use crate::peripheral::{ExitCell, PeripheralModel};
use crate::testcase::Testcase;

/// Engine tuning.
#[derive(Clone, Debug)]
pub struct EngineConfig {
    /// Instruction budget for a single run.
    pub max_steps: u64,
    /// Number of fuzzing iterations after profiling.
    pub iterations: u64,
    /// Upper bound on the fuzz input length.
    pub max_input_len: usize,
    /// RNG seed, for reproducibility.
    pub seed: u64,
    /// How many read values to try when unblocking a polled register.
    pub max_discovery_candidates: usize,
    /// Value for registers with no model.
    pub fallback: u32,
    /// Stop recording findings past this many.
    pub max_findings: usize,
    /// Treat a run as hung once this many instructions pass without new
    /// coverage. This detects tight loops long before the step budget.
    pub no_edge_limit: u64,
    /// Memory layout: explicit stack/heap/input buffers and guard holes.
    pub layout: MemoryLayout,
    /// Base address of a UART-like device that streams the fuzz input.
    pub stream_uart: Option<u32>,
    /// Address of a halt register: writing it ends a run cleanly.
    pub halt_addr: Option<u32>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_steps: 100_000,
            iterations: 2_000,
            max_input_len: 128,
            seed: 0x5eed_1234,
            max_discovery_candidates: 40,
            fallback: 0,
            max_findings: 64,
            no_edge_limit: 8_192,
            layout: MemoryLayout::default(),
            stream_uart: None,
            halt_addr: None,
        }
    }
}

/// Summary of a run.
#[derive(Clone, Debug)]
pub struct Statistics {
    pub executions: u64,
    pub edges: usize,
    pub corpus: usize,
    pub findings: usize,
    pub discovered_registers: usize,
    /// Runs that ended by writing the halt register.
    pub clean_exits: u64,
    pub elapsed: Duration,
}

/// The MMIO-aware fuzzing engine.
pub struct Engine {
    firmware: Firmware,
    config: EngineConfig,
    model: HardwareModel,
    overrides: HashMap<u32, u32>,
    coverage: Coverage,
    corpus: Vec<Input>,
    findings: Vec<Testcase>,
    mutator: Mutator,
    executions: u64,
    clean_exits: u64,
}

struct RunResult {
    steps: u64,
    final_pc: u32,
    coverage: Coverage,
    log: AccessLog,
    anomaly: Option<Anomaly>,
    /// Exit code written to the halt register, if the harness finished.
    exit: Option<u32>,
}

impl Engine {
    pub fn new(firmware: Firmware, config: EngineConfig) -> Self {
        let mutator = Mutator::new(config.seed);
        Self {
            firmware,
            config,
            model: HardwareModel::new(),
            overrides: HashMap::new(),
            coverage: Coverage::new(),
            corpus: Vec::new(),
            findings: Vec::new(),
            mutator,
            executions: 0,
            clean_exits: 0,
        }
    }

    /// Adds a starting input to the corpus (e.g. a valid seed packet).
    pub fn seed_corpus(&mut self, input: Input) {
        self.corpus.push(input);
    }

    pub fn model(&self) -> &HardwareModel {
        &self.model
    }

    pub fn config_mut(&mut self) -> &mut EngineConfig {
        &mut self.config
    }

    /// Runs one input against the profiled model, returning any anomaly.
    ///
    /// Call [`Engine::profile`] first; this is what `mmio-fuzz replay` uses to
    /// confirm a testcase reproduces.
    pub fn check(&self, input: &Input) -> Option<Anomaly> {
        self.execute(input, &self.model, &self.overrides).anomaly
    }

    /// Runs one input and returns both any anomaly and the halt exit code.
    pub fn check_outcome(&self, input: &Input) -> (Option<Anomaly>, Option<u32>) {
        let result = self.execute(input, &self.model, &self.overrides);
        (result.anomaly, result.exit)
    }

    pub fn findings(&self) -> &[Testcase] {
        &self.findings
    }

    pub fn corpus(&self) -> &[Input] {
        &self.corpus
    }

    /// Runs profiling and then fuzzing, returning aggregate statistics.
    pub fn run(&mut self) -> Statistics {
        let start = Instant::now();
        self.profile();
        for _ in 0..self.config.iterations {
            self.fuzz_once();
        }
        Statistics {
            executions: self.executions,
            edges: self.coverage.edges(),
            corpus: self.corpus.len(),
            findings: self.findings.len(),
            discovered_registers: self.model.register_count(),
            clean_exits: self.clean_exits,
            elapsed: start.elapsed(),
        }
    }

    fn fuzz_once(&mut self) {
        let mut input = self.pick_corpus();
        if self.executions > 0 {
            self.mutator.mutate(&mut input, self.config.max_input_len);
        }
        let result = self.execute(&input, &self.model, &self.overrides);
        self.executions += 1;
        if result.exit.is_some() {
            self.clean_exits += 1;
        }

        if self.coverage.count_new(&result.coverage) > 0 {
            self.coverage.merge(&result.coverage);
            self.corpus.push(input.clone());
        }

        if let Some(anomaly) = result.anomaly {
            if anomaly.kind.is_target_bug() {
                self.record_finding(&input, anomaly, result.final_pc, result.steps);
            }
        }
    }

    fn record_finding(&mut self, input: &Input, anomaly: Anomaly, final_pc: u32, steps: u64) {
        let duplicate = self
            .findings
            .iter()
            .any(|t| t.finding.kind == anomaly.kind && t.final_pc == final_pc);
        if duplicate || self.findings.len() >= self.config.max_findings {
            return;
        }
        self.findings.push(
            Testcase::new(self.firmware.hash_hex(), input, anomaly, final_pc, steps).with_repro(
                crate::testcase::ReproConfig {
                    layout: self.config.layout.clone(),
                    stream_uart: self.config.stream_uart,
                    halt_addr: self.config.halt_addr,
                },
            ),
        );
    }

    fn pick_corpus(&self) -> Input {
        if self.corpus.is_empty() {
            Input::new()
        } else {
            self.corpus[(self.executions as usize) % self.corpus.len()].clone()
        }
    }

    /// Phase 1: build a model, then unblock polling loops.
    pub fn profile(&mut self) {
        let baseline = self.execute(&Input::new(), &HardwareModel::new(), &HashMap::new());
        let mut observations = infer(&baseline.log, &InferConfig::default());

        // A second pass with the partially inferred model usually reveals more
        // traffic, because registers that were stuck at zero now move.
        let second = self.execute(&Input::new(), &observations, &HashMap::new());
        observations = merge_models(observations, infer(&second.log, &InferConfig::default()));

        let polled: Vec<u32> = polled_addresses(&observations);
        let mut overrides = HashMap::new();
        for addr in polled {
            if let Some(value) =
                self.discover_ready(addr, &observations, &overrides, &baseline.coverage)
            {
                overrides.insert(addr, value);
                if let Some(reg) = register_mut(&mut observations, addr) {
                    reg.ready_value = Some(value);
                    reg.constant_read = Some(value);
                }
            }
        }

        let third = self.execute(&Input::new(), &observations, &overrides);
        observations = merge_models(observations, infer(&third.log, &InferConfig::default()));

        self.coverage = Coverage::new();
        self.coverage.merge(&baseline.coverage);
        self.coverage.merge(&second.coverage);
        self.coverage.merge(&third.coverage);

        self.model = observations;
        self.overrides = overrides;
        self.corpus.push(Input::new());
    }

    /// Searches for a read value that unblocks a polled register.
    ///
    /// The heuristic is simple but effective: try single bits, then all ones,
    /// then any value the firmware was seen to read. The first candidate that
    /// either reaches new coverage or lets execution run noticeably longer
    /// wins.
    fn discover_ready(
        &self,
        addr: u32,
        model: &HardwareModel,
        overrides: &HashMap<u32, u32>,
        baseline: &Coverage,
    ) -> Option<u32> {
        let mut candidates: Vec<u32> = Vec::new();
        for bit in 0..32 {
            candidates.push(1u32 << bit);
        }
        candidates.push(0xffff_ffff);
        if let Some(reg) = model.register(addr) {
            for value in &reg.read_values {
                if *value != 0 {
                    candidates.push(*value);
                }
            }
        }
        candidates.dedup();

        for candidate in candidates
            .into_iter()
            .take(self.config.max_discovery_candidates)
        {
            let mut trial = overrides.clone();
            trial.insert(addr, candidate);
            let run = self.execute(&Input::new(), model, &trial);
            // Success means the firmware left the poll loop and went
            // somewhere the baseline never reached.
            if baseline.count_new(&run.coverage) > 0 {
                return Some(candidate);
            }
        }
        None
    }

    fn execute(
        &self,
        input: &Input,
        model: &HardwareModel,
        overrides: &HashMap<u32, u32>,
    ) -> RunResult {
        let mut memory = machine::build_memory(&self.firmware, &self.config.layout);

        // Materialise the fuzz input into its memory region, if any.
        if let Some((base, size)) = self.config.layout.input {
            let n = input.bytes.len().min(size as usize);
            if n > 0 {
                let _ = memory.load_image(PhysAddr::new(base), &input.bytes[..n]);
            }
        }

        let exit: ExitCell = Rc::new(Cell::new(None));
        let mut handler = PeripheralModel::new(model.clone(), input.clone())
            .with_overrides(overrides.clone())
            .with_fallback(self.config.fallback);
        if let Some(base) = self.config.stream_uart {
            handler = handler.with_stream(base, input.bytes.clone());
        }
        if let Some(addr) = self.config.halt_addr {
            handler = handler.with_halt(addr, exit.clone());
        }
        memory.set_mmio_handler(handler);

        let log = Rc::new(RefCell::new(AccessLog::new()));
        let sink = log.clone();
        memory.set_observer(FnObserver(move |access: &Access| {
            sink.borrow_mut().push(*access);
        }));

        let mut core = CortexM::new(Cpu::new(), memory);
        let mut coverage = Coverage::new();
        let mut steps = 0u64;
        let mut anomaly: Option<Anomaly> = None;
        let mut exit_code: Option<u32> = None;

        if let Err(error) = core.reset() {
            anomaly = Some(Anomaly::new(
                AnomalyKind::EmulatorError,
                0,
                0,
                format!("reset failed: {error}"),
            ));
        } else {
            let mut since_new_edge = 0u64;
            loop {
                let pc = core.pc().raw();
                let edges_before = coverage.edges();
                coverage.record(pc);
                if coverage.edges() > edges_before {
                    since_new_edge = 0;
                } else {
                    since_new_edge += 1;
                }

                match core.step() {
                    Ok(()) => {
                        steps += 1;
                        if let Some(code) = exit.get() {
                            exit_code = Some(code);
                            break;
                        }
                        if core.cpu.xpsr.exception_number() == 3 {
                            anomaly = Some(Anomaly::new(
                                AnomalyKind::HardFault,
                                pc,
                                steps,
                                "HardFault exception taken",
                            ));
                            break;
                        }
                        if since_new_edge >= self.config.no_edge_limit {
                            anomaly = Some(Anomaly::new(
                                AnomalyKind::Hang,
                                pc,
                                steps,
                                "no new coverage (tight loop)",
                            ));
                            break;
                        }
                        if steps >= self.config.max_steps {
                            anomaly = Some(Anomaly::new(
                                AnomalyKind::Hang,
                                pc,
                                steps,
                                "step budget exhausted",
                            ));
                            break;
                        }
                    }
                    Err(error) => {
                        anomaly = Some(self.classify_error(error, pc, steps));
                        break;
                    }
                }
            }
        }

        let final_pc = core.pc().raw();
        drop(core);

        let log = Rc::try_unwrap(log)
            .map(|cell| cell.into_inner())
            .unwrap_or_else(|rc| rc.borrow().clone());

        RunResult {
            steps,
            final_pc,
            coverage,
            log,
            anomaly,
            exit: exit_code,
        }
    }

    /// Classifies an execution error, distinguishing a guard hole from a
    /// generic unmapped address.
    fn classify_error(&self, error: CoreError, pc: u32, steps: u64) -> Anomaly {
        match error {
            CoreError::Unmapped { addr, .. } if self.config.layout.is_guard(addr.raw()) => {
                Anomaly::new(
                    AnomalyKind::GuardHit,
                    pc,
                    steps,
                    format!("guard hit at {addr}"),
                )
            }
            other => anomaly_from_error(other, pc, steps),
        }
    }
}

fn anomaly_from_error(error: CoreError, pc: u32, steps: u64) -> Anomaly {
    match error {
        CoreError::InvalidOpcode { opcode, .. } => Anomaly::new(
            AnomalyKind::InvalidOpcode,
            pc,
            steps,
            format!("invalid opcode {opcode:#010x}"),
        ),
        CoreError::Unmapped { addr, .. } => Anomaly::new(
            AnomalyKind::UnmappedAccess,
            pc,
            steps,
            format!("unmapped access at {addr}"),
        ),
        CoreError::OutOfBounds { addr, .. } => Anomaly::new(
            AnomalyKind::OutOfBounds,
            pc,
            steps,
            format!("out-of-bounds access at {addr}"),
        ),
        CoreError::StepLimit(_) => Anomaly::new(AnomalyKind::Hang, pc, steps, "step limit"),
        other => Anomaly::new(AnomalyKind::EmulatorError, pc, steps, other.to_string()),
    }
}

fn polled_addresses(model: &HardwareModel) -> Vec<u32> {
    let mut addrs: Vec<u32> = model
        .blocks
        .iter()
        .flat_map(|block| &block.registers)
        .filter(|reg| reg.polled)
        .map(|reg| reg.addr)
        .collect();
    addrs.sort_unstable();
    addrs.dedup();
    addrs
}

fn register_mut(model: &mut HardwareModel, addr: u32) -> Option<&mut RegisterModel> {
    let base = HardwareModel::block_base(addr);
    model
        .blocks
        .iter_mut()
        .find(|block| block.base == base)
        .and_then(|block| block.registers.iter_mut().find(|reg| reg.addr == addr))
}

fn merge_models(mut dst: HardwareModel, src: HardwareModel) -> HardwareModel {
    for block in src.blocks {
        for reg in block.registers {
            match register_mut(&mut dst, reg.addr) {
                Some(existing) => merge_register(existing, &reg),
                None => dst.block_for(reg.addr).registers.push(reg),
            }
        }
    }
    dst
}

fn merge_register(dst: &mut RegisterModel, src: &RegisterModel) {
    dst.reads += src.reads;
    dst.writes += src.writes;
    dst.read_values.extend(src.read_values.iter().copied());
    dst.write_values.extend(src.write_values.iter().copied());
    dst.widen_to(src.width);
    dst.polled |= src.polled;
    dst.poll_sites.extend(src.poll_sites.iter().copied());
    dst.read_to_clear |= src.read_to_clear;
    dst.write_to_clear_hint |= src.write_to_clear_hint;
    dst.ready_mask |= src.ready_mask;
    if dst.constant_read.is_none() {
        dst.constant_read = src.constant_read;
    }
    if dst.ready_value.is_none() {
        dst.ready_value = src.ready_value;
    }
    if dst.constant_read.is_none() && dst.reads >= 2 && dst.read_values.len() == 1 {
        dst.constant_read = dst.read_values.iter().next().copied();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_sane() {
        let cfg = EngineConfig::default();
        assert!(cfg.max_steps > 0);
        assert!(cfg.max_input_len >= 4);
        assert!(cfg.max_findings > 0);
    }

    #[test]
    fn merging_unions_register_observations() {
        let mut a = RegisterModel::new(0x4000_0000);
        a.reads = 1;
        a.read_values.insert(0x1);
        let mut b = RegisterModel::new(0x4000_0000);
        b.reads = 1;
        b.read_values.insert(0x2);

        let merged = merge_models(
            HardwareModel::from_registers(vec![a]),
            HardwareModel::from_registers(vec![b]),
        );
        let reg = merged.register(0x4000_0000).unwrap();
        assert_eq!(reg.reads, 2);
        assert!(reg.read_values.contains(&0x1) && reg.read_values.contains(&0x2));
    }
}
