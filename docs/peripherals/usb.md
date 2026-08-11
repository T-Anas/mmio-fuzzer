# USB device

* Category: Serial

## Typical registers

- Endpoint registers expose a buffer and a status with valid/ready handshakes.
- Firmware manipulates many endpoint registers in sequence.

## Patterns the inference can recognise

- Many-register sequences
- Handshake bits

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
