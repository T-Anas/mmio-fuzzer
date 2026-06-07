//! Shared vocabulary for the `mmio-fuzz` toolchain.
//!
//! `mmio-core` deliberately stays free of execution or fuzzing logic. It only
//! describes the things every other crate agrees on: address spaces, access
//! kinds, the errors that can cross a backend boundary, and the trait that
//! lets a backend report MMIO traffic back to the tracer.

pub mod access;
pub mod addr;
pub mod error;

pub use access::{Access, AccessKind, AccessWidth};
pub use addr::PhysAddr;
pub use error::{CoreError, Result};
