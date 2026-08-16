# Analog comparator

* Category: Analog

## Typical registers

- A status register shows which input is higher; an interrupt enable and a flag are separate.
- The flag is write-one-to-clear.

## Patterns the inference can recognise

- Read-only status
- Write-one-to-clear flag

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
