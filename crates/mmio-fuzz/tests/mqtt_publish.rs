//! Run the real MQTT-C deserialiser on Cortex-M0 and check both a benign
//! PUBLISH and the malformed one from CVE-2026-54412.
//!
//! The harness lives in `targets/mqtt-c/harness`; `xtask build-targets`
//! regenerates the committed ELF.

use mmio_fuzz::{AnomalyKind, Engine, EngineConfig, Firmware, Input, MemoryLayout};

const MQTT_ELF: &[u8] = include_bytes!("../../../targets/prebuilt/mqtt_publish.elf");

fn engine() -> Engine {
    let firmware = Firmware::from_elf_bytes(MQTT_ELF).expect("parse mqtt harness");
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
    };
    let config = EngineConfig {
        layout,
        halt_addr: Some(0x4000_f000),
        max_steps: 200_000,
        no_edge_limit: 100_000,
        ..EngineConfig::default()
    };
    Engine::new(firmware, config)
}

#[test]
fn benign_publish_parses_and_exits() {
    // PUBLISH QoS0, remaining length 5, topic "t", payload "hi".
    let seed = vec![0x30, 0x05, 0x00, 0x01, b't', b'h', b'i'];
    let (anomaly, exit) = engine().check_outcome(&Input::from_vec(seed));
    assert!(anomaly.is_none(), "benign packet faulted: {anomaly:?}");
    assert_eq!(exit, Some(0), "harness should reach the halt register");
}

#[test]
fn malformed_topic_length_faults() {
    // PUBLISH with topic_name_size = 0xFFFF but remaining_length = 7.
    let evil = vec![0x30, 0x07, 0xff, 0xff, 0x00, 0x00, 0x00];
    let (anomaly, exit) = engine().check_outcome(&Input::from_vec(evil));
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
