# UART / USART

* Category: Serial

## Typical registers

- A status register exposes a transmit-empty and a receive-not-empty bit; firmware spins on them.
- A data register holds transmitted or received bytes, often with a read-to-clear receive flag.
- A control register configures baud, word length, parity and enable.

## Patterns the inference can recognise

- Poll loops on both status bits
- Read-to-clear receive flag
- Write-only data register

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
