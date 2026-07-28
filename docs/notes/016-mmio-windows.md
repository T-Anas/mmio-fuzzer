# The default MMIO windows

ARMv6-M defines the peripheral window at 0x4000_0000..0x6000_0000 and the private peripheral bus at 0xE000_0000..0xE010_0000.

`is_mmio_address` uses exactly those ranges. Vendors that stray can be handled by widening the configuration.
