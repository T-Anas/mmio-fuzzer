# BKPT

`bkpt` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `BKPT #imm8` |
| **Encoding** | `1011 1110 imm8` |
| **Operation** | Breakpoint |
| **Flags** | none |

## Notes

Halts debugging; useful as a fixture terminator.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
