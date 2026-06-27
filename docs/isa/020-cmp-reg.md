# CMP (register)

`cmp-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `CMP Rn, Rm` |
| **Encoding** | `0100 0010 10 Rm Rd` |
| **Operation** | Rn - Rm, discard result |
| **Flags** | NZCV |

## Notes

Subtract and set flags only.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
