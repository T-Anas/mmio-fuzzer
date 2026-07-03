# LDR (literal)

`ldr-literal` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDR Rt, label` |
| **Encoding** | `0100 1 Rt imm8*4` |
| **Operation** | Rt = mem32[Align(PC,4)+imm] |
| **Flags** | none |

## Notes

PC-relative constant pool load.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
