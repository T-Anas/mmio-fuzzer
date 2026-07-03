# MVNS

`mvns` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `MVNS Rd, Rm` |
| **Encoding** | `0100 0011 11 Rm Rd` |
| **Operation** | Rd = ~Rm |
| **Flags** | NZ |

## Notes

Bitwise not.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
