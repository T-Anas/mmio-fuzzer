# LDRH (register)

`ldrh-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDRH Rt, [Rn, Rm]` |
| **Encoding** | `0101 101 Rm Rn Rt` |
| **Operation** | Rt = ZeroExtend(mem16[Rn+Rm]) |
| **Flags** | none |

## Notes

Halfword load, register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
