# Watchdog

* Category: Reset

## Typical registers

- A reload register is written with a magic value to pet the watchdog.
- Reading the register may return a status or key, so read and write values differ.

## Patterns the inference can recognise

- Read/write value asymmetry
- Magic-key writes

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
