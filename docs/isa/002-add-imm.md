# ADD (immediate, 3-bit)

`add-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADDS Rd, Rn, #imm3` |
| **Encoding** | `0001 110 imm3 Rn Rd` |
| **Operation** | Rd = Rn + imm3 |
| **Flags** | NZCV |

## Notes

Immediate is zero-extended to 32 bits.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
