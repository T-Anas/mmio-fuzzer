# ADC

* Category: Analog

## Typical registers

- A conversion is started by writing a control register, then firmware polls an end-of-conversion bit.
- The result register shares the data register with the channel selection in some parts.

## Patterns the inference can recognise

- Start-then-poll handshake
- Ready-bit discovery

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
