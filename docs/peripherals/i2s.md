# I2S / audio

* Category: Serial

## Typical registers

- Same shape as SPI with a sample-based data register.
- A half-empty flag drives DMA or polling.

## Patterns the inference can recognise

- Fifo flag poll

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
