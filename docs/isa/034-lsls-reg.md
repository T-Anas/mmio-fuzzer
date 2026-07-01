# LSLS (register)

`lsls-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `LSLS Rd, Rm, Rs` |
| **Encoding** | `0100 0001 00 Rs Rd` |
| **Operation** | Rd = Rd << Rs |
| **Flags** | NZC |

## Notes

Register-controlled shift.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
