# PUSH

`push` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `PUSH {reglist, LR}` |
| **Encoding** | `1011 010 M reglist` |
| **Operation** | store list to stack |
| **Flags** | none |

## Notes

Bit 8 stores LR.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
