# BLX (register)

`blx-reg` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `BLX Rm` |
| **Encoding** | `0100 0111 1 Rm 000` |
| **Operation** | LR = PC+2|1 ; PC = Rm |
| **Flags** | none |

## Notes

Call through a register.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
