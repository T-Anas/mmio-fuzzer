//! Reproducible testcases.

use serde::{Deserialize, Serialize};

use crate::anomaly::{Anomaly, AnomalyKind};
use crate::input::Input;
use crate::machine::MemoryLayout;
use mmio_infer::HardwareModel;

/// The machine configuration a finding was produced under.
///
/// Testcases must be reproducible, and for parser targets that means more than
/// the input bytes: the memory layout, streaming device and halt register all
/// affect behaviour, so they travel with the finding.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReproConfig {
    pub layout: MemoryLayout,
    #[serde(default)]
    pub stream_uart: Option<u32>,
    #[serde(default)]
    pub halt_addr: Option<u32>,
}

/// Everything needed to replay a finding.
///
/// A testcase is self-describing: it carries the firmware hash (so you can
/// confirm you have the right image), the exact input, and the observation
/// that made the run interesting.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Testcase {
    pub firmware_hash: String,
    pub input: Vec<u8>,
    pub finding: Anomaly,
    pub final_pc: u32,
    pub steps: u64,
    /// Machine configuration used for the run, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repro: Option<ReproConfig>,
    /// Optional snapshot of the model in force when the finding was made.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub model: Option<HardwareModel>,
}

impl Testcase {
    pub fn new(
        firmware_hash: impl Into<String>,
        input: &Input,
        finding: Anomaly,
        final_pc: u32,
        steps: u64,
    ) -> Self {
        Self {
            firmware_hash: firmware_hash.into(),
            input: input.bytes.clone(),
            finding,
            final_pc,
            steps,
            repro: None,
            model: None,
        }
    }

    /// Attaches the machine configuration used for the run.
    pub fn with_repro(mut self, repro: ReproConfig) -> Self {
        self.repro = Some(repro);
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("testcase is serialisable")
    }

    pub fn from_json(text: &str) -> serde_json::Result<Self> {
        serde_json::from_str(text)
    }

    /// Suggested filename, e.g. `crash-ab12cd34-deadbeef.mmf`.
    pub fn filename(&self) -> String {
        let kind = match self.finding.kind {
            AnomalyKind::HardFault => "hardfault",
            AnomalyKind::InvalidOpcode => "badop",
            AnomalyKind::UnmappedAccess => "unmapped",
            AnomalyKind::GuardHit => "guard",
            AnomalyKind::OutOfBounds => "oob",
            AnomalyKind::Hang => "hang",
            AnomalyKind::EmulatorError => "emu",
        };
        let input = Input::from_vec(self.input.clone());
        format!("{kind}-{}-{}.mmf", self.firmware_hash, input.short_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_and_names() {
        let input = Input::from_vec(vec![1, 2, 3, 4]);
        let finding = Anomaly::new(AnomalyKind::HardFault, 0x0800_0100, 42, "boom");
        let case = Testcase::new("deadbeef", &input, finding, 0x0800_0100, 42);
        let json = case.to_json();
        let back = Testcase::from_json(&json).unwrap();
        assert_eq!(back.final_pc, 0x0800_0100);
        assert!(case.filename().starts_with("hardfault-deadbeef-"));
        assert!(case.filename().ends_with(".mmf"));
    }
}
