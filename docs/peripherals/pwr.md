# Power control

* Category: System

## Typical registers

- A control register selects low-power modes; a status register shows the active mode.

## Patterns the inference can recognise

- Write-then-read status

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
