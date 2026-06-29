# ASRS (register)

`asrs-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ASRS Rd, Rm, Rs` |
| **Encoding** | `0100 0001 00 Rs Rd` |
| **Operation** | Rd = Rd >> Rs |
| **Flags** | NZC |

## Notes

The shift amount is Rs[7:0].

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
