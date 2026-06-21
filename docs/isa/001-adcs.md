# ADCS

`adcs` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADCS Rd, Rm` |
| **Encoding** | `0100 0001 01 Rm Rd` |
| **Operation** | Rd = Rd + Rm + C |
| **Flags** | NZCV |

## Notes

Add with carry; part of the register data-processing group.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
