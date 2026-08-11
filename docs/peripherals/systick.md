# SysTick

* Category: Timing

## Typical registers

- Four registers: control/status, reload value, current value and calibration.
- The current value counts down and the count flag is set when it reaches zero.

## Patterns the inference can recognise

- Changing current-value reads
- Count-flag poll

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
