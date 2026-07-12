# STRH (immediate)

`strh-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STRH Rt, [Rn, #imm5*2]` |
| **Encoding** | `1000 0 imm5 Rn Rt` |
| **Operation** | mem16[Rn+imm] = Rt[15:0] |
| **Flags** | none |

## Notes

Halfword store.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
