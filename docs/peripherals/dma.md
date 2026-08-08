# DMA controller

* Category: Data movement

## Typical registers

- Channel configuration registers hold source, destination and count.
- A status register exposes a transfer-complete flag that is write-one-to-clear.

## Patterns the inference can recognise

- Long command sequences
- Completion poll then clear

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
