//! Turning an MMIO access trace into a model of the hardware.
//!
//! This crate is the intellectual centre of `mmio-fuzz`. Given the reads and
//! writes a firmware performs against peripheral addresses, it reconstructs
//! enough of the register behaviour to answer the question that matters during
//! fuzzing: *what should this read return so the firmware makes progress?*
//!
//! The inference is deliberately conservative. We would rather leave a
//! register marked "unknown" than confidently feed the firmware a value that
//! sends it down a path no silicon would allow.

pub mod infer;
pub mod log;
pub mod model;
pub mod register;

pub use infer::{infer, InferConfig};
pub use log::AccessLog;
pub use model::{HardwareModel, PeripheralBlock};
pub use register::{AccessStyle, RegisterModel};
