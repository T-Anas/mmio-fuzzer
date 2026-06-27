# CMP (immediate)

`cmp-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `CMP Rn, #imm8` |
| **Encoding** | `0010 1 Rn imm8` |
| **Operation** | Rn - imm8, discard result |
| **Flags** | NZCV |

## Notes

Immediate is zero-extended.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
