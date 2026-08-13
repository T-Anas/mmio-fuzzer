# Reset and clock control

* Category: System

## Typical registers

- Firmware writes enable bits and polls a ready bit for each clock domain.
- Enable and ready bits live in the same register, so reads change after writes.

## Patterns the inference can recognise

- Enable-then-ready poll
- Read/write in one register

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
