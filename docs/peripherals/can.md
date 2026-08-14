# CAN

* Category: Serial

## Typical registers

- Mailbox registers hold identifier, data and length; a request bit starts transmission.
- A status register exposes transmit-request-complete, cleared by writing one.

## Patterns the inference can recognise

- Mailbox write sequence
- Completion clear

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
