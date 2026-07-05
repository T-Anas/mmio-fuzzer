# RSBS

`rsbs` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `RSBS Rd, Rm, #0` |
| **Encoding** | `0100 0010 01 Rm Rd` |
| **Operation** | Rd = 0 - Rm |
| **Flags** | NZCV |

## Notes

Negate.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
