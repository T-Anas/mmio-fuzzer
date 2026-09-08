# External RAM

* Address range: `0x6000_0000 - 0x7FFF_FFFF`

## Facts

Off-chip memory, often through a memory controller.
Not used by the current fixtures.

## In mmio-fuzz

Treated as ordinary RW memory if mapped at all.
