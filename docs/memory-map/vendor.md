# Vendor system region

* Address range: `0xE010_0000 - 0xFFFF_FFFF`

## Facts

Vendor-specific system control within the PPB area.
Not architected and not mapped by default.

## In mmio-fuzz

Widen the MMIO windows in configuration to include it if a fixture needs it.
