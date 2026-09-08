# SRAM region

* Address range: `0x2000_0000 - 0x3FFF_FFFF`

## Facts

Writable data: stack, heap and `.data`/`.bss`.
The initial stack pointer comes from the first vector word.

## In mmio-fuzz

Regions are RW and start zeroed. Stack writes are ordinary RAM traffic.
