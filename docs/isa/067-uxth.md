# UXTH

`uxth` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `UXTH Rd, Rm` |
| **Encoding** | `1011 0010 10 Rm Rd` |
| **Operation** | Rd = ZeroExtend(Rm[15:0]) |
| **Flags** | none |

## Notes

Unsigned halfword extend.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
