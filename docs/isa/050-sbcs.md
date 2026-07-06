# SBCS

`sbcs` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SBCS Rd, Rm` |
| **Encoding** | `0100 0001 10 Rm Rd` |
| **Operation** | Rd = Rd - Rm - (1-C) |
| **Flags** | NZCV |

## Notes

Subtract with carry.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
