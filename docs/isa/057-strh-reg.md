# STRH (register)

`strh-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STRH Rt, [Rn, Rm]` |
| **Encoding** | `0101 001 Rm Rn Rt` |
| **Operation** | mem16[Rn+Rm] = Rt[15:0] |
| **Flags** | none |

## Notes

Halfword store, register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
