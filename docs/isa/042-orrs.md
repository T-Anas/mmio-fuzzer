# ORRS

`orrs` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ORRS Rd, Rm` |
| **Encoding** | `0100 0011 00 Rm Rd` |
| **Operation** | Rd = Rd | Rm |
| **Flags** | NZ |

## Notes

Bitwise or.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
