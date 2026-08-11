# AES / crypto accelerator

* Category: Security

## Typical registers

- Key and data are written to input registers, a start bit is set, then a done bit is polled.
- Output registers return the result only after completion.

## Patterns the inference can recognise

- Multi-register command
- Done poll

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
