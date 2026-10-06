# Firmware hunt: method and surface portfolio

> Internal working document. No external disclosure without explicit approval.

## Objective

Find real memory-safety or state bugs in **narrow, under-fuzzed attack
surfaces** of embedded radio/network stacks, using the MMIO-aware fuzzer. We do
not fuzz whole stacks; each campaign targets one protocol corner reachable
through a peripheral.

## Rules

* Real upstream code, pinned by commit, compiled for Cortex-M0.
* Find bugs from benign seeds; no known CVE is used as the objective.
* A candidate is only kept if it replays deterministically and is not a
  harness artefact (guard/redzone misconfiguration, unmapped-but-expected
  memory, emulator bug).
* Prefer surfaces with no OSS-Fuzz coverage and no published CVE.

## Engine capabilities

Implemented:

| Capability | What it unlocks |
| --- | --- |
| Shadow memory / redzones (`--redzone`) | precise out-of-bounds inside mapped RAM |
| SysTick + NVIC + exception return | firmware driven by interrupts |
| Stateful chunk device (`--stream-uart`, `--stream-width`, `--stream-adaptive`) | FIFO/mailbox data delivery (CAN, UART) |

Still pending:

| Capability | Why it matters |
| --- | --- |
| Snapshot/restore + persistent mode | longer campaigns, higher throughput |
| Host ASan cross-check harness | independent confirmation of a finding |

## Surface portfolio

| # | Surface | Project | Entry | Hypothesis | Coverage |
| --- | --- | --- | --- | --- | --- |
| A | SDO block transfer | CANopenNode | CAN FIFO | sequence/length handling → OOB, state confusion | no OSS-Fuzz, no CVE |
| B | LSS | CANopenNode | CAN FIFO | identity/state bugs (cf. RT-Labs C-Open CVEs) | no CVE |
| C | Transport Protocol reassembly | a small open J1939 stack | CAN FIFO | reassembly index/size → OOB | almost none |
| D | MEI / Read Device Identification (0x2B/0x0E) | FreeMODBUS | UART | object offset/length handling | obscure corner |
| E | FUOTA fragmented data block | lmix / arduino-lmic | radio downlink | attacker-chosen `frag_size` → OOB | class under-audited outside Zephyr |
| F | cookie / HelloVerifyRequest + epoch | tinydtls | UDP/radio | state/epoch confusion | partially fuzzed |
| G | variable data structures | an MBUS/DLMS parser | UART | structure length handling | very little |

First wave: **A, C, D**.

## Per-surface recipe

1. Vendor the source under `targets/<name>/` (commit pinned, licence kept).
2. Minimal Cortex-M0 port: peripheral entry modelled with the chunk device /
   CAN controller, timers via SysTick, completion via the halt register.
3. Seeds: valid frames for the surface; dictionary of lengths, indices,
   COB-IDs, CRC fields.
4. Campaign: `mmio-fuzz run` in release, 15–30 min, coverage as the metric.
5. Triage: dedup by (kind, PC), minimise, replay, then validate against a
   host ASan build of the same functions.

## Status

* Engine capabilities 1–3 are implemented and tested.
* MQTT-C calibration (earlier campaign) remains the proof the pipeline finds
  real bugs; it is not part of this portfolio.
* Surface A/C/D harnesses are the current work. CANopenNode needs a full port
  (heap, OD, LSS, storage) and is the heaviest; FreeMODBUS and a small J1939 TP
  reassembler are lighter and are the practical starting points.
