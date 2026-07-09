# SUB (SP minus immediate)

`sub-sp` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `SUB SP, #imm7*4` |
| **Encoding** | `1011 0000 1 imm7` |
| **Operation** | SP = SP - imm |
| **Flags** | none |

## Notes

Deallocate stack.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
