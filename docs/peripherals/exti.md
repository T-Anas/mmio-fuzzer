# External interrupt controller

* Category: Interrupts

## Typical registers

- Mask and pending registers; pending bits are write-one-to-clear.

## Patterns the inference can recognise

- Write-one-to-clear pending

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
