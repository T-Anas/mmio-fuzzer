# Code region

* Address range: `0x0000_0000 - 0x1FFF_FFFF`

## Facts

Executable memory, usually aliased to flash at reset.
The vector table lives at the base before any relocation.

## In mmio-fuzz

We map the reset vector down to zero so `reset` works for flash-linked images.
