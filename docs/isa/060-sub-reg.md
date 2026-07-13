# SUB (register)

`sub-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SUBS Rd, Rn, Rm` |
| **Encoding** | `0001 101 Rm Rn Rd` |
| **Operation** | Rd = Rn - Rm |
| **Flags** | NZCV |

## Notes

Three-register subtract.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
