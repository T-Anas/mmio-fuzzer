# LDR (immediate)

`ldr-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDR Rt, [Rn, #imm5*4]` |
| **Encoding** | `0110 1 imm5 Rn Rt` |
| **Operation** | Rt = mem32[Rn+imm] |
| **Flags** | none |

## Notes

Word load with scaled offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
