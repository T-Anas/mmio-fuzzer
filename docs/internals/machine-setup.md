# Machine setup

`build_memory` composes a `MemoryMap` from a firmware image:

* one region per loadable segment, flash below `0x2000_0000` and RAM above;
* a default RAM window at `0x2000_0000`;
* the peripheral window at `0x4000_0000`;
* the private peripheral bus at `0xE000_0000`;
* a vector alias at zero when the image is linked elsewhere.

Overlaps with firmware-defined regions are tolerated: the firmware wins,
because the image knows its own memory layout better than our defaults.

After the map is built, the image is applied and, if needed, the first 256
bytes of the lowest segment are copied to address zero so `reset` can read SP
and the reset vector.
