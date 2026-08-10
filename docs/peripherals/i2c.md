# I2C

* Category: Serial

## Typical registers

- Start, address and stop bits live in a control register; a status register exposes busy and ACK.
- Firmware sequences control writes and status polls, a good test of dependency inference.

## Patterns the inference can recognise

- Multi-write command sequence
- Busy poll

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
