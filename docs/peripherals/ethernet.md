# Ethernet MAC

* Category: Networking

## Typical registers

- Descriptor rings live in RAM; firmware configures base addresses and polls ownership bits.
- DMA writes descriptors the firmware then reads, a rich source of dependencies.

## Patterns the inference can recognise

- Descriptor DMA
- Ownership-bit poll

## Fuzzing notes

This page describes the shapes `mmio-infer` looks for, not a specific part.
Register addresses and bit positions vary between vendors; the model learns
them from the firmware's own accesses rather than from a datasheet.
