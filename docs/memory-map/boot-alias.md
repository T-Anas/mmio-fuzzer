# Boot alias

* Address range: `0x0000_0000 (aliased)`

## Facts

At reset the core fetches SP and PC from address zero, which hardware aliases to the boot memory.
Later, firmware may relocate the vector table by writing VTOR on parts that have it.

## In mmio-fuzz

We copy the first 256 bytes of the lowest segment to zero to reproduce the alias.
