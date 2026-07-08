# MULS

`muls` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `MULS Rd, Rm, Rd` |
| **Encoding** | `0100 0011 01 Rm Rd` |
| **Operation** | Rd = Rd * Rm |
| **Flags** | NZ |

## Notes

32-bit low multiply.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
