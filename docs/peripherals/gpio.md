# GPIO

* Category: Digital

## Typical registers

- Per-pin direction and data registers, plus set and clear aliases.
- A data register read reflects input pins; writes drive output pins.

## Patterns the inference can recognise

- Read/write register pairs
- Write-one-to-set/clear aliases

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
