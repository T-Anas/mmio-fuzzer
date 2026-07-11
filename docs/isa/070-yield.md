# YIELD

`yield` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `YIELD` |
| **Encoding** | `1011 1111 0001 0000` |
| **Operation** | yield |
| **Flags** | none |

## Notes

Hint; modelled as a no-op.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
