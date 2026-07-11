# STR (register)

`str-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `STR Rt, [Rn, Rm]` |
| **Encoding** | `0101 000 Rm Rn Rt` |
| **Operation** | mem32[Rn+Rm] = Rt |
| **Flags** | none |

## Notes

Word store with register offset.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
