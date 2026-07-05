# REVSH

`revsh` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `REVSH Rd, Rm` |
| **Encoding** | `1011 1010 11 Rm Rd` |
| **Operation** | reverse low halfword, sign-extend |
| **Flags** | none |

## Notes

Signed byte-reversed halfword.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
