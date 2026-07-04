# LDRB (register)

`ldrb-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDRB Rt, [Rn, Rm]` |
| **Encoding** | `0101 110 Rm Rn Rt` |
| **Operation** | Rt = ZeroExtend(mem8[Rn+Rm]) |
| **Flags** | none |

## Notes

Byte load, register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
