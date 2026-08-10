# SPI

* Category: Serial

## Typical registers

- Firmware writes a byte to a data register and polls a busy flag until the transfer completes.
- A status register usually has a busy bit and an overrun bit.

## Patterns the inference can recognise

- Single-bit busy poll
- Write-then-poll command sequence

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
