# LDRH (immediate)

`ldrh-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDRH Rt, [Rn, #imm5*2]` |
| **Encoding** | `1000 1 imm5 Rn Rt` |
| **Operation** | Rt = ZeroExtend(mem16[Rn+imm]) |
| **Flags** | none |

## Notes

Halfword load.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
