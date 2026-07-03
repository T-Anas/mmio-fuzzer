# EORS

`eors` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `EORS Rd, Rm` |
| **Encoding** | `0100 0000 01 Rm Rd` |
| **Operation** | Rd = Rd ^ Rm |
| **Flags** | NZ |

## Notes

Exclusive or.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
