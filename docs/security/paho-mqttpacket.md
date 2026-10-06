# Eclipse Paho MQTTPacket — memory-safety findings

> Internal report. Not yet disclosed. Reachability assessed, PoCs validated
> under host AddressSanitizer and under the mmio-fuzz Cortex-M0 emulator.

Component: `paho.mqtt.embedded-c`, directory `MQTTPacket`.
Commit tested: `6035ea2d4922bb7558b444fb2a051743f3f1974b`.
License: EPL-2.0 / EDL-1.0.

Two issues were found by the `mmio-fuzz` pipeline and validated independently
under ASan.

---

## Finding 1 — deserialisers ignore `buflen` (out-of-bounds read)

**CWE-125.** The buffer deserialisers (`MQTTDeserialize_publish`,
`_subscribe`, `_unsubscribe`, `_connack`, `_suback`, `_unsuback`, `_ack`) read
the MQTT *remaining length* into `mylen` and compute

```c
enddata = curdata + mylen;     /* mylen is attacker-controlled */
```

`mylen` is **never** compared against the caller-supplied `buflen`. Every
subsequent bound check is expressed against `enddata`, so it is meaningless.
`readMQTTLenString` then accepts a topic length up to the bogus `enddata`,
which walks `curdata` past the buffer; the next `readInt`/`readChar`
dereferences the wild pointer.

### Reachability (verified)

| Caller | Bounds `rem_len`? | Exposed |
| --- | --- | --- |
| `MQTTPacket_read` | yes (`rem_len + len > buflen` → exit) | no |
| `MQTTClient-C` `readPacket` | yes (`rem_len > readbuf_size - len` → exit, fix #96) | no |
| Paho C++ `MQTT::Client::readPacket` | yes (`rem_len > MAX_MQTT_PACKET_SIZE - len`) | no |
| **public `MQTTDeserialize_*` API** | n/a | **yes** |
| **`MQTTFormat_toClientString` / `toServerString`** | no | **yes** |

So the bundled clients' normal receive loop is **not** exploitable. The
exposed surface is the public deserialiser API — the documented way to parse a
received packet, used directly by embedded MQTT **brokers / gateways** and by
applications that log packets with `MQTTFormat_*` — whenever the caller passes
a buffer whose in-band remaining length exceeds `buflen`. The `buflen`
parameter exists precisely to prevent this and is ignored.

**Severity:** medium for a server/broker that parses untrusted client packets
with this API (remote, unauthenticated at the framing layer); low for the
bundled clients, which pre-bound the length.

### PoC

* Host ASan: `targets/paho/poc/poc_subscribe.c` → stack-buffer-overflow READ
  in `readInt` ← `readMQTTLenString` ← `MQTTDeserialize_subscribe`.
* Cortex-M0 under mmio-fuzz: `crates/mmio-fuzz/tests/paho_mqtt.rs`. Input
  region followed by a redzone and guard holes; the malformed SUBSCRIBE
  yields `UnmappedAccess at 0x20014008`. The engine also rediscovers it
  autonomously from a benign seed.
* Archived `.mmf`: `targets/paho/findings/*.mmf` (replay reproduces).

### Fix

Validate the remaining length against the buffer before use, in every
deserialiser, and compare string lengths against the *buffer* end:

```c
if (mylen < 0 || (size_t)mylen > (size_t)buflen - (size_t)(curdata - buf)) {
    goto exit;
}
```

---

## Finding 2 — off-by-one write in `MQTTFormat_toServerString`

**CWE-787.** `MQTTFormat_toServerString` ends with

```c
strbuf[strbuflen] = '\0';
```

`strbuflen` is the **size** of `strbuf` (it is passed to `snprintf` as the
size just above), so this writes one byte past the caller's buffer. It happens
on **every call**, including on a valid packet, and there is no bounds check.

`MQTTFormat_toClientString` does not have this line.

**Severity:** low (one NUL byte), but deterministic; depending on the caller's
stack/heap layout the overwritten byte may be a length field, a saved pointer,
or a stack canary.

### PoC

`targets/paho/poc/poc_format_offbyone.c` → ASan reports a 1-byte
stack-buffer-overflow WRITE at `MQTTFormat.c:265` on a benign PINGRESP.

### Fix

Remove the line; `snprintf` already NUL-terminates. If a guarantee is wanted,
use `strbuf[strbuflen - 1] = '\0';`.

---

## Method

Found with `mmio-fuzz`: the real `MQTTPacket` sources compiled for Cortex-M0
behind a UART-style input buffer, executed on the pure-Rust ARMv6-M
interpreter, with shadow-memory redzones and guard holes. Confirmed by an
independent host ASan build of the same functions. See
`docs/security/hunt-plan.md` for the method and capabilities.

## Credit / status

Internal report. No external disclosure performed. Suggested reporting
channel and a ready-to-send message: `docs/security/paho-report-to-send.md`.
