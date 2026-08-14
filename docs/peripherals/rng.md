# RNG

* Category: Security

## Typical registers

- A control register enables the generator; a status register has a data-ready bit.
- The data register returns a fresh random value on every read.

## Patterns the inference can recognise

- Ready poll
- High-entropy reads

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
