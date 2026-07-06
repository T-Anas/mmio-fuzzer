# LSLS (immediate)

`lsls-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LSLS Rd, Rm, #imm5` |
| **Encoding** | `0000 0 imm5 Rm Rd` |
| **Operation** | Rd = Rm << imm |
| **Flags** | NZC |

## Notes

Logical left shift.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
