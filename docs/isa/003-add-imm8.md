# ADD (immediate, 8-bit)

`add-imm8` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADDS Rd, #imm8` |
| **Encoding** | `0011 0 Rd imm8` |
| **Operation** | Rd = Rd + imm8 |
| **Flags** | NZCV |

## Notes

The common small-constant add.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
