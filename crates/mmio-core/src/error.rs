use thiserror::Error;

use crate::access::{AccessKind, AccessWidth};
use crate::addr::PhysAddr;

/// Errors that can cross a backend or bus boundary.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("unmapped {kind} {width} access at {addr}")]
    Unmapped {
        addr: PhysAddr,
        kind: AccessKind,
        width: AccessWidth,
    },

    #[error("misaligned {width} access at {addr}")]
    Misaligned { addr: PhysAddr, width: AccessWidth },

    #[error("invalid opcode {opcode:#010x} at pc {pc:#010x}")]
    InvalidOpcode { opcode: u32, pc: u32 },

    #[error("execution limit reached after {0} instructions")]
    StepLimit(u64),

    #[error("bus error: {0}")]
    Bus(String),

    #[error("backend does not provide {0}")]
    Unsupported(&'static str),
}

impl CoreError {
    /// True when the error looks like a firmware-visible fault rather than a
    /// limitation of our own tooling. The fuzzer treats these as findings.
    pub fn is_target_fault(&self) -> bool {
        matches!(
            self,
            CoreError::Unmapped { .. }
                | CoreError::Misaligned { .. }
                | CoreError::InvalidOpcode { .. }
        )
    }
}

/// Convenience result alias used across the workspace.
pub type Result<T> = core::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_faults_are_classified() {
        let e = CoreError::Unmapped {
            addr: PhysAddr::new(0x4000_0000),
            kind: AccessKind::Read,
            width: AccessWidth::Word,
        };
        assert!(e.is_target_fault());

        let e = CoreError::Unsupported("dma");
        assert!(!e.is_target_fault());
    }
}
