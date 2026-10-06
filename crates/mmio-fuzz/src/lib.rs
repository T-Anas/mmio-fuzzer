//! Coverage-guided fuzzing that understands MMIO.
//!
//! The engine runs a firmware image inside the [`mmio_emu`] interpreter, but
//! instead of poking memory it drives the *outside world*: the values returned
//! by peripheral reads are the fuzz input. A pass of [`mmio_infer`] over the
//! resulting trace builds a hardware model, and that model seeds the value
//! generator so the firmware can actually get past its init code.

pub mod anomaly;
pub mod coverage;
pub mod engine;
pub mod input;
pub mod machine;
pub mod mutate;
pub mod peripheral;
pub mod testcase;

pub use anomaly::{Anomaly, AnomalyKind};
pub use coverage::Coverage;
pub use engine::{Engine, EngineConfig, Statistics};
pub use input::Input;
pub use machine::{build_core, build_memory, Firmware, MachineError, MemoryLayout};
pub use peripheral::{ExitCell, PeripheralModel};
pub use testcase::{ReproConfig, Testcase};
