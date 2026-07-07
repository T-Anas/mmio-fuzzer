# MOV (immediate)

`mov-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `MOVS Rd, #imm8` |
| **Encoding** | `0010 0 Rd imm8` |
| **Operation** | Rd = imm8 |
| **Flags** | NZ |

## Notes

Loads a small constant.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
