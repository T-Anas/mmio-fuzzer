# ADD (high register)

`add-high` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `ADD Rd, Rm` |
| **Encoding** | `0100 0100 DN Rm Rd` |
| **Operation** | Rd = Rd + Rm |
| **Flags** | none |

## Notes

Two-register form; can name a high register.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
