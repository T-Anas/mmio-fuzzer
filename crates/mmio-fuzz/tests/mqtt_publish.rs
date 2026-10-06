//! MQTT-C calibration and negative control.
//!
//! * The benign and malformed packets are checked against the unpatched and
//!   patched builds (from `xtask build-targets`).
//! * The engine is then run from a single benign PUBLISH seed and must
//!   rediscover the out-of-bounds read on its own — the CVE-2026-54412 class
//!   of bug — without being handed a proof of concept.

use mmio_fuzz::{AnomalyKind, Engine, EngineConfig, Firmware, Input, MemoryLayout};

const MQTT_ELF: &[u8] = include_bytes!("../../../targets/prebuilt/mqtt_publish.elf");
const MQTT_FIXED_ELF: &[u8] = include_bytes!("../../../targets/prebuilt/mqtt_publish_fixed.elf");

fn engine_for(elf: &[u8], iterations: u64) -> Engine {
    let firmware = Firmware::from_elf_bytes(elf).expect("parse mqtt harness");
    let layout = MemoryLayout {
        ram: Some((0x2000_0000, 0x1000)),
        stack: Some((0x2000_8000, 0x1000)),
        heap: Some((0x2000_2000, 0x1000)),
        input: Some((0x2000_4000, 0x1000)),
        guards: vec![
            (0x2000_1000, 0x1000),
            (0x2000_3000, 0x1000),
            (0x2000_5000, 0x3000),
        ],
        redzones: Vec::new(),
    };
    let config = EngineConfig {
        layout,
        halt_addr: Some(0x4000_f000),
        iterations,
        max_steps: 40_000,
        no_edge_limit: 4_000,
        ..EngineConfig::default()
    };
    Engine::new(firmware, config)
}

fn benign_publish() -> Input {
    // PUBLISH QoS0, remaining length 5, topic "t", payload "hi".
    Input::from_vec(vec![0x30, 0x05, 0x00, 0x01, b't', b'h', b'i'])
}

fn malformed_publish() -> Input {
    // topic_name_size = 0xFFFF while remaining_length = 7.
    Input::from_vec(vec![0x30, 0x07, 0xff, 0xff, 0x00, 0x00, 0x00])
}

#[test]
fn benign_publish_parses_and_exits() {
    let (anomaly, exit) = engine_for(MQTT_ELF, 0).check_outcome(&benign_publish());
    assert!(anomaly.is_none(), "benign packet faulted: {anomaly:?}");
    assert_eq!(exit, Some(0), "harness should reach the halt register");
}

#[test]
fn malformed_topic_length_faults() {
    let (anomaly, exit) = engine_for(MQTT_ELF, 0).check_outcome(&malformed_publish());
    assert!(exit.is_none());
    let anomaly = anomaly.expect("malformed packet should fault");
    assert!(
        matches!(
            anomaly.kind,
            AnomalyKind::GuardHit | AnomalyKind::UnmappedAccess
        ),
        "unexpected anomaly: {anomaly:?}"
    );
}

#[test]
fn patched_build_rejects_the_malformed_packet() {
    let (anomaly, exit) = engine_for(MQTT_FIXED_ELF, 0).check_outcome(&malformed_publish());
    assert!(anomaly.is_none(), "patched build faulted: {anomaly:?}");
    assert_eq!(exit, Some(0), "patched build should reject cleanly");
}

#[test]
fn fuzzer_rediscovers_the_bug_from_a_benign_seed() {
    let mut engine = engine_for(MQTT_ELF, 6_000);
    engine.seed_corpus(benign_publish());
    engine.run();

    assert!(
        engine
            .findings()
            .iter()
            .any(|finding| finding.finding.kind.is_target_bug()),
        "the engine did not rediscover the out-of-bounds read"
    );
}
