# MOV (register)

`mov-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `MOV Rd, Rm` |
| **Encoding** | `0100 0110 DN Rm Rd` |
| **Operation** | Rd = Rm |
| **Flags** | none |

## Notes

Does not update flags.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
