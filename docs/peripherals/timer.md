# General-purpose timer

* Category: Timing

## Typical registers

- A counter register increments freely; firmware reads it for delays or writes it to reload.
- Control enables the timer; a status register raises an update flag that is cleared by writing one.

## Patterns the inference can recognise

- Continuously changing reads
- Write-one-to-clear status flag

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
