# STR (immediate)

`str-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STR Rt, [Rn, #imm5*4]` |
| **Encoding** | `0110 0 imm5 Rn Rt` |
| **Operation** | mem32[Rn+imm] = Rt |
| **Flags** | none |

## Notes

Word store with scaled offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
