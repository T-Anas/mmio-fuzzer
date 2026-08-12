# RTC

* Category: Timing

## Typical registers

- Time and date registers can be shadowed; a write to a shadow register commits on a read.
- A second-change flag is write-one-to-clear.

## Patterns the inference can recognise

- Shadow register sequence
- Write-one-to-clear flag

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
