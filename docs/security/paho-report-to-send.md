# How to report the Paho MQTTPacket findings

## Where

The project's `SECURITY.md` points at the Eclipse Foundation process. Report
privately, not in a public issue.

1. **Eclipse Foundation Vulnerability Reporting Tracker** (confidential GitLab
   issue — first choice):
   <https://gitlab.eclipse.org/security/vulnerability-reports/-/issues/new?issuable_template=new_vulnerability>
2. **Eclipse Foundation Security Team**: `security@eclipse-foundation.org`
   (policy: <https://www.eclipse.org/security/policy/>)
3. If you prefer a GitHub-native path, the repository also supports private
   advisories: <https://github.com/eclipse-paho/paho.mqtt.embedded-c/security/advisories/new>
4. The Eclipse Foundation is a CNA, so it can assign a CVE. If the report is
   not acknowledged within ~14 days, escalate to `security@eclipse-foundation.org`
   and, failing that, request a CVE from MITRE
   (<https://www.cve.org/ResourcesSupport/ReportRequest>).

State your disclosure preference (coordinated 90 days, or public immediately)
and whether you want credit.

## What to say (paste-ready)

> **Subject:** Out-of-bounds read in MQTTPacket deserialisers and off-by-one
> write in `MQTTFormat_toServerString`
>
> **Affected:** `paho.mqtt.embedded-c`, `MQTTPacket` directory, tested at
> commit `6035ea2d4922bb7558b444fb2a051743f3f1974b`.
>
> **1. Deserialisers ignore `buflen` — out-of-bounds read (CWE-125).**
> `MQTTDeserialize_publish` / `_subscribe` / `_unsubscribe` (and the other
> buffer deserialisers) decode the MQTT remaining length into `mylen` and set
> `enddata = curdata + mylen` without ever checking `mylen` against the
> caller-supplied `buflen`. `readMQTTLenString` then validates strings against
> this bogus `enddata`, so a crafted packet walks `curdata` past the buffer
> and the next `readInt`/`readChar` reads out of bounds.
>
> Reachability: the bundled clients are safe — `MQTTPacket_read` and both
> `MQTTClient-C` and the C++ `MQTT::Client` bound `rem_len` before calling the
> deserialiser. The exposed paths are the public `MQTTDeserialize_*` API
> (used directly by embedded brokers/gateways) and the bundled
> `MQTTFormat_toClientString` / `MQTTFormat_toServerString` helpers, whenever
> the buffer's in-band remaining length exceeds `buflen`.
>
> PoC (host ASan): buffer `82 FF FF FF 7F 00 01 00 02 61 62` to
> `MQTTDeserialize_subscribe(..., sizeof(buf))` →
> `stack-buffer-overflow READ in readInt ← readMQTTLenString ← MQTTDeserialize_subscribe`.
> A build of the same functions for Cortex-M0 and run under an ARMv6-M
> emulator confirms it (the read lands outside the mapped buffer). The fuzzer
> rediscovers it from a single benign SUBSCRIBE seed.
>
> **2. Off-by-one write in `MQTTFormat_toServerString` (CWE-787).**
> The function ends with `strbuf[strbuflen] = '\0';` where `strbuflen` is the
> buffer *size* (it is passed to `snprintf` as the size on the line above).
> This writes one byte past the caller's buffer on every call, including on a
> valid packet. `MQTTFormat_toClientString` does not have this line.
>
> PoC (host ASan): call `MQTTFormat_toServerString(out, sizeof(out), pingresp, 2)`
> with a stack buffer → `1-byte stack-buffer-overflow WRITE at MQTTFormat.c:265`.
>
> **Suggested fixes:**
> * In every deserialiser, after decoding the remaining length:
>   `if (mylen < 0 || (size_t)mylen > (size_t)buflen - (size_t)(curdata - buf)) goto exit;`
>   and compare string lengths against the buffer end, not `curdata + mylen`.
> * In `MQTTFormat_toServerString`, remove `strbuf[strbuflen] = '\0';`
>   (`snprintf` already terminates) or use `strbuf[strbuflen - 1] = '\0';`.
>
> I can provide the ASan harnesses and the emulator testcases. I request
> coordinated disclosure and can be credited if you wish.

## Evidence to attach

* `targets/paho/poc/poc_subscribe.c` (finding 1, host ASan)
* `targets/paho/poc/poc_format_offbyone.c` (finding 2, host ASan)
* `targets/paho/findings/*.mmf` (mmio-fuzz reproducers)
* `crates/mmio-fuzz/tests/paho_mqtt.rs` (autonomous rediscovery)
