# Operational amplifier

* Category: Analog

## Typical registers

- Configuration registers select gain and input; there is little to read back.

## Patterns the inference can recognise

- Write-only configuration

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
