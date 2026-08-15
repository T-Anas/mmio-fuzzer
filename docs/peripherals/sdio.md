# SDIO / SDMMC

* Category: Storage

## Typical registers

- Command arguments are written, a command register is set, then firmware polls response bits.
- Responses are read from several registers.

## Patterns the inference can recognise

- Command/response sequence

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
