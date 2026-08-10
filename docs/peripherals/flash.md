# Flash controller

* Category: Memory

## Typical registers

- Unlock keys are written, then program or erase bits are set, then firmware polls busy.
- A status register exposes a busy bit and error flags, cleared by writing one.

## Patterns the inference can recognise

- Unlock key sequence
- Busy poll
- Write-one-to-clear errors

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
