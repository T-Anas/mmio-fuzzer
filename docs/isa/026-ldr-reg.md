# LDR (register)

`ldr-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDR Rt, [Rn, Rm]` |
| **Encoding** | `0101 100 Rm Rn Rt` |
| **Operation** | Rt = mem32[Rn+Rm] |
| **Flags** | none |

## Notes

Word load with register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
