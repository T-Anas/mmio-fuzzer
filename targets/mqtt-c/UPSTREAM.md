# Vendored target: MQTT-C

* Upstream: <https://github.com/LiamBindle/MQTT-C>
* Commit: `7a986a68ebea63921d4aab20a9d1b26a8b5f8c9d` (2022-10-27)
* License: MIT (see `LICENSE` in this directory)

## Files

| File | Purpose |
| --- | --- |
| `src/mqtt.c` | Upstream deserialiser/serialiser (verbatim) |
| `include/mqtt.h` | Upstream header (verbatim) |
| `harness/` | Cortex-M0 harness written for mmio-fuzz |

## Why this target

MQTT-C is a small, dependency-free MQTT client used on microcontrollers. Its
`mqtt_unpack_*` functions parse bytes received from the network, which is
exactly the kind of untrusted input a firmware fuzzer should exercise. The
deserialiser is reachable from a single memory buffer, so it runs under the
ARMv6-M interpreter with a tight, guard-protected memory layout.

This copy is pinned for reproducibility of the hunting campaign. The harness
files are ours; the upstream sources are unmodified.
