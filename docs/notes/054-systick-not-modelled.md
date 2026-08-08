# SysTick is not modelled yet

The private peripheral bus is mapped as MMIO, so SysTick register accesses are observed and inferred like any other peripheral.

There is no timer that raises an interrupt on its own yet. Firmware that busy-waits on SysTick count works; firmware that relies on the exception does not.
