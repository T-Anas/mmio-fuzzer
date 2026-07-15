# SXTH

`sxth` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SXTH Rd, Rm` |
| **Encoding** | `1011 0010 00 Rm Rd` |
| **Operation** | Rd = SignExtend(Rm[15:0]) |
| **Flags** | none |

## Notes

Signed halfword extend.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
