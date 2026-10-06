//! Eclipse Paho MQTTPacket deserialisers on Cortex-M0.
//!
//! The library trusts the MQTT "remaining length" field without checking it
//! against the caller's buffer length. A crafted SUBSCRIBE (or PUBLISH) makes
//! it read past the input buffer. We assert both the benign path and the
//! out-of-bounds path, then let the fuzzer rediscover it.

use mmio_fuzz::{AnomalyKind, Engine, EngineConfig, Firmware, Input, MemoryLayout};

const PAHO_ELF: &[u8] = include_bytes!("../../../targets/prebuilt/paho_mqtt.elf");

fn layout() -> MemoryLayout {
    MemoryLayout::tight()
        .with_ram(0x2000_0000, 0x1000)
        .with_stack(0x2000_8000, 0x1000)
        .with_input(0x2000_4000, 0x1000)
        .with_redzone(0x2000_5000, 0x1000)
        .with_guard(0x2000_6000, 0x2000)
}

fn engine(iterations: u64) -> Engine {
    let firmware = Firmware::from_elf_bytes(PAHO_ELF).expect("parse paho image");
    let config = EngineConfig {
        layout: layout(),
        halt_addr: Some(0x4000_f000),
        iterations,
        max_steps: 5_000,
        no_edge_limit: 2_000,
        ..EngineConfig::default()
    };
    Engine::new(firmware, config)
}

/// A well-formed SUBSCRIBE for topic "ab". The remaining length is honest.
fn benign_subscribe() -> Input {
    // 82 07 00 01 00 02 'a' 'b' 00
    Input::from_vec(vec![0x82, 0x07, 0x00, 0x01, 0x00, 0x02, b'a', b'b', 0x00])
}

/// A SUBSCRIBE whose remaining length claims 0x0FFF_FFFF bytes and whose topic
/// length is 0xFFFF, so the deserialiser walks past the buffer.
fn malformed_subscribe() -> Input {
    Input::from_vec(vec![0x82, 0xff, 0xff, 0xff, 0x7f, 0x00, 0x01, 0xff, 0xff])
}

#[test]
fn benign_subscribe_parses_and_exits() {
    let (anomaly, exit) = engine(0).check_outcome(&benign_subscribe());
    assert!(anomaly.is_none(), "benign packet faulted: {anomaly:?}");
    assert_eq!(exit, Some(0));
}

#[test]
fn malformed_subscribe_reads_out_of_bounds() {
    let (anomaly, exit) = engine(0).check_outcome(&malformed_subscribe());
    assert!(exit.is_none(), "should not reach the halt register");
    let anomaly = anomaly.expect("malformed packet should fault");
    assert!(
        matches!(
            anomaly.kind,
            AnomalyKind::OutOfBounds | AnomalyKind::GuardHit | AnomalyKind::UnmappedAccess
        ),
        "unexpected anomaly: {anomaly:?}"
    );
}

#[test]
fn fuzzer_rediscovers_the_out_of_bounds_from_a_benign_seed() {
    let mut engine = engine(8_000);
    engine.seed_corpus(benign_subscribe());
    engine.run();
    assert!(
        engine
            .findings()
            .iter()
            .any(|f| f.finding.kind.is_target_bug()),
        "the engine did not rediscover the out-of-bounds read"
    );
}
