# ADD (SP plus immediate)

`add-sp-imm` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADD Rd, SP, #imm8*4 / ADD SP, #imm7*4` |
| **Encoding** | `1010 1 Rd imm8 / 1011 0000 0 imm7` |
| **Operation** | Rd = SP + imm |
| **Flags** | none |

## Notes

SP-relative addressing and stack adjustment.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
