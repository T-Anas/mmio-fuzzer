# Private peripheral bus

* Address range: `0xE000_0000 - 0xE00F_FFFF`

## Facts

NVIC, SysTick, System Control Block and debug registers.
Architected, not vendor-specific.

## In mmio-fuzz

Mapped as System kind and observed like any other MMIO.
