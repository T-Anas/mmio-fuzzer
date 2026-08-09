# DAC

* Category: Analog

## Typical registers

- A data register holds the output value; a trigger bit starts conversion.
- Reads usually return the last written value or zero, making the register look constant if only written.

## Patterns the inference can recognise

- Write-only register

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
