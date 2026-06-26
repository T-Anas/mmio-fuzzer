# MQTT-C deserialiser hunt — internal report

> **Internal only.** Per the engagement rules, findings from this campaign are
> not disclosed externally. Nothing here has been reported to a vendor or
> MITRE.

## Scope

Target: **MQTT-C** `mqtt.c`, commit
`7a986a68ebea63921d4aab20a9d1b26a8b5f8c9d` (2022-10-27), ESP32/Cortex-M class
MQTT client used on microcontrollers.

Goal: hunt for a previously unknown memory-safety bug in the response
deserialiser, using the MMIO-aware fuzzer on a real Cortex-M0 build.

Method constraints:

* run the *real* upstream C on our ARMv6-M interpreter, not on the host;
* start from benign seeds only — the fuzzer must discover any crash itself;
* use a patched build as a negative control.

## Harness

`targets/mqtt-c/harness/` compiles upstream `mqtt.c` for `cortex-m0` behind a
minimal freestanding platform layer, plus a small `main` that calls
`mqtt_unpack_response` (which dispatches on the packet type) and then acts as a
typical application: copying a PUBLISH payload and walking a SUBACK's return
codes. Input lives at `0x2000_4000`; the input and scratch buffers are
surrounded by unmapped guard holes so an out-of-bounds access becomes a
`GuardHit`.

Build: `cargo xtask build-targets`. Prebuilt images are committed.

## How the fuzzer was driven

```sh
./target/release/mmio-fuzz run targets/prebuilt/mqtt_publish.elf \
  --iterations 300000 --max-steps 50000 \
  --input-mem 0x20004000:0x1000 \
  --ram 0x20000000:0x1000 --stack 0x20008000:0x1000 --heap 0x20002000:0x1000 \
  --guard 0x20001000:0x1000 --guard 0x20003000:0x1000 --guard 0x20005000:0x3000 \
  --halt-addr 0x4000F000 \
  --seed-file seeds/publish.bin --seed-file seeds/suback.bin \
  --seed-file seeds/connack.bin --seed-file seeds/puback.bin \
  --seed-file seeds/unsuback.bin --seed-file seeds/pingresp.bin \
  --out out/
```

The CLI is not required: `crates/mmio-fuzz/tests/mqtt_publish.rs` reproduces
the calibration in-process.

## Calibration (positive control)

The engine, seeded with a single well-formed PUBLISH, rediscovers the
out-of-bounds read behind **CVE-2026-54412** within a few thousand executions:

* benign PUBLISH, `remaining_length=5`, `topic_name_size=1` → harness reaches
  the halt register, `exit = Some(0)`;
* malformed PUBLISH, `remaining_length=7`, `topic_name_size=0xFFFF` → the
  underflowed `application_message_size` drives the consumer `memcpy` out of
  bounds → `GuardHit` / `UnmappedAccess`.

Negative control: the same image built with the bounds check
(`topic_name_size <= remaining_length - 2`) rejects the malformed packet and
exits cleanly. This rules out a harness artefact.

## Deep campaign result

300 000 executions, ~129 s in release, six seeds, 284 336 clean exits.

| Kind | Faulting PC | Packet type | Root cause |
| --- | --- | --- | --- |
| GuardHit | `0x0800_0042` | PUBLISH | publish length underflow |
| UnmappedAccess | `0x0800_0042` | PUBLISH | publish length underflow |
| GuardHit | `0x0800_0082` | PUBLISH (QoS>0) | publish length underflow → `__mqtt_unpack_uint16` |
| UnmappedAccess | `0x0800_0082` | PUBLISH (QoS>0) | publish length underflow |
| Hang | `0x0800_0044` | PUBLISH | bounded SUBACK/consumer loop |
| Hang | `0x0800_005e` | SUBACK | bounded consumer loop |

Every **memory-safety** finding is the same defect: `mqtt_unpack_publish_response`
computes `application_message_size = remaining_length - topic_name_size - 2`
(or `-4`) without checking that `topic_name_size` fits inside
`remaining_length`.

## Why no *new* bug was found

We looked for a second defect and could not find one in this surface, and we
can explain why:

* `mqtt_unpack_fixed_header` rejects a packet when
  `bufsz < remaining_length`, so `remaining_length` cannot exceed the caller's
  buffer.
* `mqtt_unpack_connack_response`, `mqtt_unpack_pubxxx_response` and
  `mqtt_unpack_unsuback_response` require `remaining_length == 2`.
* `mqtt_unpack_suback_response` derives `num_return_codes = remaining_length - 2`
  and points at the buffer; because `remaining_length <= bufsz`, a consumer
  walking the codes stays inside the buffer. Our SUBACK seeds produce only a
  bounded loop, which the engine reports as a hang, not a memory fault.

The publish deserialiser is the only place where an *in-band* length is trusted
beyond the packet, which is exactly why it is the only exploitable parser here.

## Reproducing a finding

```sh
./target/release/mmio-fuzz replay out/guard-*.mmf targets/prebuilt/mqtt_publish.elf
```

Each `.mmf` stores the firmware hash, the exact input and the machine layout,
so replay rebuilds the same machine and re-checks the same anomaly.

## Next steps

1. **Paho embedded-C** (`MQTTDeserialize_*`, `MQTTPacket_decodeBuf`): larger
   surface, global `bufptr` state, candidates for a genuine 0-day.
2. **Serialisers**: fuzz `mqtt_pack_*` with fuzzed field values; unchecked
   output lengths would be a different bug class.
3. **Shadow-memory sanitizer** to catch OOB that stays inside a mapped region,
   which guard holes alone miss.
4. Longer campaigns and cross-checking against a host build under ASan.

## Status

No previously unknown vulnerability reported in this pass. The pipeline's
ability to find *this* class of bug is demonstrated and reproducible; the
negative control confirms the finding is attributable to the missing check.
