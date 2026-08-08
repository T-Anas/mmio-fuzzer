# NVIC

* Category: Interrupts

## Typical registers

- Set-enable and clear-enable register pairs, a pending set and clear pair, and per-priority bytes.
- Enable registers are write-one-to-set; clear registers write-one-to-clear.

## Patterns the inference can recognise

- Write-one-to-set/clear pairs
- Narrow priority accesses

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
