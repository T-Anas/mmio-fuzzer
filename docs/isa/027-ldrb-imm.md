# LDRB (immediate)

`ldrb-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDRB Rt, [Rn, #imm5]` |
| **Encoding** | `0111 1 imm5 Rn Rt` |
| **Operation** | Rt = ZeroExtend(mem8[Rn+imm]) |
| **Flags** | none |

## Notes

Unsigned byte load.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
