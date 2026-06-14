//! Findings the engine can report.
//!
//! Everything here is something a human would want to look at: the firmware
//! faulted, hit an instruction we cannot emulate, accessed unmapped memory,
//! or simply never made progress.

use serde::{Deserialize, Serialize};

/// Category of an anomaly.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyKind {
    /// The core took a HardFault exception.
    HardFault,
    /// The decoder met an encoding we do not model.
    InvalidOpcode,
    /// The firmware tried to touch an address nothing maps.
    UnmappedAccess,
    /// Execution never terminated within the step budget.
    Hang,
    /// The engine itself hit an internal error.
    EmulatorError,
}

impl AnomalyKind {
    /// Whether this is a target bug rather than a tooling gap.
    pub fn is_target_bug(self) -> bool {
        matches!(
            self,
            AnomalyKind::HardFault | AnomalyKind::UnmappedAccess | AnomalyKind::Hang
        )
    }
}

/// A concrete observation worth reproducing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Anomaly {
    pub kind: AnomalyKind,
    pub pc: u32,
    pub detail: String,
    pub steps: u64,
}

impl Anomaly {
    pub fn new(kind: AnomalyKind, pc: u32, steps: u64, detail: impl Into<String>) -> Self {
        Self {
            kind,
            pc,
            detail: detail.into(),
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_target_bugs() {
        assert!(AnomalyKind::HardFault.is_target_bug());
        assert!(AnomalyKind::Hang.is_target_bug());
        assert!(!AnomalyKind::InvalidOpcode.is_target_bug());
    }
}
