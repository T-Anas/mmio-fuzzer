# SUB (immediate, 3-bit)

`sub-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SUBS Rd, Rn, #imm3` |
| **Encoding** | `0001 111 imm3 Rn Rd` |
| **Operation** | Rd = Rn - imm3 |
| **Flags** | NZCV |

## Notes

Subtract small constant.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
