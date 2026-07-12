# STRB (immediate)

`strb-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STRB Rt, [Rn, #imm5]` |
| **Encoding** | `0111 0 imm5 Rn Rt` |
| **Operation** | mem8[Rn+imm] = Rt[7:0] |
| **Flags** | none |

## Notes

Byte store.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
