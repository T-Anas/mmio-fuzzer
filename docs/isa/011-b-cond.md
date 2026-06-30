# B (conditional)

`b-cond` — ARMv6-M instruction.

| | |
| --- | --- |
| **Syntax** | `B<cond> label` |
| **Encoding** | `1101 cond imm8*2` |
| **Operation** | if cond then PC += imm |
| **Flags** | none |

## Notes

Condition codes are the standard ARM set.

## Implementation

See `crates/mmio-emu/src/decode.rs` for the encoding and
`crates/mmio-emu/src/exec.rs` for the semantics. Decoding is tested with
hand-assembled encodings; semantics are covered by the instruction tests.
