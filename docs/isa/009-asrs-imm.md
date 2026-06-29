# ASRS (immediate)

`asrs-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ASRS Rd, Rm, #imm5` |
| **Encoding** | `0001 0 imm5 Rm Rd` |
| **Operation** | Rd = Rm >> imm (arithmetic) |
| **Flags** | NZC |

## Notes

imm5 of zero means no shift and no flag change.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
