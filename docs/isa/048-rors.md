# RORS

`rors` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `RORS Rd, Rm, Rs` |
| **Encoding** | `0100 0001 11 Rs Rd` |
| **Operation** | Rd = rotate_right(Rd, Rs) |
| **Flags** | NZC |

## Notes

Carry receives the last bit rotated out.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
