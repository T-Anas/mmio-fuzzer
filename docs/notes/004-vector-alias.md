# Aliasing the vector table to zero

Firmware is linked at 0x0800_0000 but the core fetches reset vectors from zero.

We copy the first 256 bytes of the lowest segment to a small region at zero, mirroring the hardware boot alias. Without it, every reset fails.
