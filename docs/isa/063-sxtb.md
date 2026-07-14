# SXTB

`sxtb` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SXTB Rd, Rm` |
| **Encoding** | `1011 0010 01 Rm Rd` |
| **Operation** | Rd = SignExtend(Rm[7:0]) |
| **Flags** | none |

## Notes

Signed byte extend.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
