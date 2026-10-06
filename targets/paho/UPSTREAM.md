# Vendored target: Eclipse Paho MQTTPacket (embedded-C)

* Upstream: <https://github.com/eclipse-paho/paho.mqtt.embedded-c>
* Commit: `6035ea2d4922bb7558b444fb2a051743f3f1974b`
* License: Eclipse Public License 2.0 / Eclipse Distribution License 1.0
  (see `../LICENSE`)

## Files

| Path | Purpose |
| --- | --- |
| `mqttpacket/` | Upstream `MQTTPacket/src` (verbatim) |
| `harness/` | Cortex-M0 harness written for mmio-fuzz |
| `poc/` | Host AddressSanitizer reproducer for the OOB read |

## Why this target

The MQTTPacket deserialisers parse bytes received from an MQTT peer. They are
buffer-oriented, dependency-free, and used on microcontrollers, which makes
them reachable from a UART/radio peripheral and a natural fit for the fuzzer.

The deserialisers trust the MQTT *remaining length* field: they compute
`enddata = curdata + mylen` from it and never check `mylen` against the
caller-supplied buffer length. A crafted packet therefore drives out-of-bounds
reads inside the library.
