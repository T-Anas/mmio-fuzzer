# SUB (immediate, 8-bit)

`sub-imm8` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SUBS Rd, #imm8` |
| **Encoding** | `0011 1 Rd imm8` |
| **Operation** | Rd = Rd - imm8 |
| **Flags** | NZCV |

## Notes

Subtract constant.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
