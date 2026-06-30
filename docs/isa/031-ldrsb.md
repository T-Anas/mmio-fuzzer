# LDRSB

`ldrsb` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LDRSB Rt, [Rn, Rm]` |
| **Encoding** | `0101 011 Rm Rn Rt` |
| **Operation** | Rt = SignExtend(mem8[Rn+Rm]) |
| **Flags** | none |

## Notes

Signed byte load.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
