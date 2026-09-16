# DMA modelling

* Status: proposed


## Motivation

Descriptor rings are written to RAM and read by a device.

## Proposal

Propose a simple DMA engine that copies between regions and raises completion bits.

## Open questions

How to validate this without regressing current behaviour?
