# Unchecked MQTT remaining length → out-of-bounds read in Eclipse Paho MQTTPacket

> Internal finding. Not disclosed externally. Reproduction and analysis below.

## Summary

The MQTT deserialisers in Eclipse Paho **MQTTPacket** (the `paho.mqtt.embedded-c`
repository) derive the end of a packet from the MQTT *remaining length* field
without ever checking it against the caller-supplied buffer length. A crafted
packet therefore makes the library read past the buffer.

* Component: `MQTTPacket/src` (`MQTTPacket.c`, `MQTTSubscribeServer.c`, and the
  other deserialisers)
* Commit tested: `6035ea2d4922bb7558b444fb2a051743f3f1974b`
* Class: CWE-125 out-of-bounds read
* Severity: **medium** — remote (MQTT peer), no authentication required at the
  framing layer; on a memory-constrained MCU it is a clean denial of service,
  and it can disclose adjacent memory.

## Root cause

`MQTTDeserialize_subscribe` (and friends) do:

```c
rc = MQTTPacket_decodeBuf(curdata, &mylen);   /* attacker-controlled */
curdata += rc;
enddata = curdata + mylen;                    /* no check against buflen */
...
while (curdata < enddata) {
    if (!readMQTTLenString(&topicFilters[*count], &curdata, enddata)) goto exit;
    ...
}
```

`readMQTTLenString` only validates a string against `enddata`, which is itself
derived from the untrusted `mylen`, so the bound is meaningless. A large
`mylen` plus a large topic length walks `curdata` far past the buffer; the
next `readInt` dereferences that address.

The same pattern exists in `MQTTDeserialize_publish`,
`MQTTDeserialize_unsubscribe`, and the other buffer deserialisers, which all
call `MQTTPacket_decodeBuf` and trust its result.

## Reproduction

### Host (AddressSanitizer)

```sh
cd targets/paho/poc
gcc -g -fsanitize=address -I ../mqttpacket poc_subscribe.c \
    ../mqttpacket/MQTTPacket.c ../mqttpacket/MQTTSubscribeServer.c -o poc
./poc
# AddressSanitizer: stack-buffer-overflow
#   #0 readInt            MQTTPacket.c:128
#   #1 readMQTTLenString  MQTTPacket.c:223
#   #2 MQTTDeserialize_subscribe MQTTSubscribeServer.c:64
```

### Cortex-M0 firmware under mmio-fuzz

`crates/mmio-fuzz/tests/paho_mqtt.rs` runs the real library on the
ARMv6-M interpreter. The input region is followed by a redzone and guard
holes, so any read past the buffer is reported by the emulator:

```
malformed_subscribe_reads_out_of_bounds:
  UnmappedAccess at 0x20014008 (read)
```

The engine also **rediscovers the bug on its own** from a single benign
SUBSCRIBE seed (`fuzzer_rediscovers_the_out_of_bounds_from_a_benign_seed`),
without being given the proof of concept.

## Impact

An MQTT peer controls the bytes parsed here:

* an MQTT **client** parses CONNACK/PUBLISH/SUBACK/UNSUBACK from the broker;
* an MQTT **server** parses SUBSCRIBE/UNSUBSCRIBE/PUBLISH from clients.

A single malformed packet can drive the parser out of bounds. On the
emulated Cortex-M0 target the read hits unmapped memory, which on real
hardware is a HardFault → device reset / DoS. Where the read stays in
mapped memory it can leak adjacent data through whatever the application
does with the parsed payload.

## Fix

Validate the remaining length against the supplied buffer before using it,
e.g. right after decoding it:

```c
if (mylen < 0 || mylen > buflen - (curdata - buf)) {
    goto exit;
}
```

and apply the same guard in every deserialiser. String length checks should
compare against the *buffer* end, not against `curdata + mylen`.

## Status

Found by the mmio-fuzz pipeline (Cortex-M0 harness + shadow/guard memory).
Reproduced independently under host ASan. Internal report only; no external
disclosure performed.
