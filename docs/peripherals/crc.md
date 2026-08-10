# CRC unit

* Category: Checksum

## Typical registers

- Firmware writes data and reads the accumulated result; a control register may select polynomial.
- The data register reads back as the running CRC, so it changes with each write.

## Patterns the inference can recognise

- Write-then-read changing register

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
