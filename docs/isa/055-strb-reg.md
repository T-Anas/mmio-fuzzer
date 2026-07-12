# STRB (register)

`strb-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STRB Rt, [Rn, Rm]` |
| **Encoding** | `0101 010 Rm Rn Rt` |
| **Operation** | mem8[Rn+Rm] = Rt[7:0] |
| **Flags** | none |

## Notes

Byte store, register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
