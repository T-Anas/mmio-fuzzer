# ADD (register)

`add-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADDS Rd, Rn, Rm` |
| **Encoding** | `0001 100 Rm Rn Rd` |
| **Operation** | Rd = Rn + Rm |
| **Flags** | NZCV |

## Notes

Three-register add.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
