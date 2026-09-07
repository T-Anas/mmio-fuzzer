# External device

* Address range: `0x8000_0000 - 0x9FFF_FFFF`

## Facts

A second device window on some parts.
Rarely used on the small targets we emulate.

## In mmio-fuzz

The `uart_demo` fixture deliberately stores here to trigger an unmapped access.
