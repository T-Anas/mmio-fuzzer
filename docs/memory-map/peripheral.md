# Peripheral region

* Address range: `0x4000_0000 - 0x5FFF_FFFF`

## Facts

Memory-mapped device registers.
This is the window the whole project is about.

## In mmio-fuzz

Mapped as Peripheral kind so every access is offered to the model and observed.
