# mmio-fuzz

`mmio-fuzz` is a firmware fuzzer for bare-metal **ARM Cortex-M** targets that
does not need the real board — nor a complete datasheet — to make progress.

## The problem

Firmware talks to the outside world through memory-mapped I/O. On a Cortex-M
that means reads and writes to `0x4000_0000` and above reach a UART, a timer,
a GPIO block, a DMA engine... When you emulate such a firmware without the
peripheral models, every device read returns `0x0`. Firmware quickly gets
stuck in `while !(reg & READY) {}` loops, or takes paths it would never take
on silicon, and your coverage map looks empty.

`mmio-fuzz` attacks that gap. It watches the MMIO traffic a firmware actually
performs, infers a lightweight model of each register from the access
patterns, and feeds *plausible* values back during fuzzing instead of zeros.

## Pipeline

```text
 Cortex-M firmware
        |
   Emulation (pure-Rust ARMv6-M core)
        |
  MMIO interception  -> access journal
        |
  Register inference  (width, constants, R/W, W1C, ready bits...)
        |
   Hardware model
        |
 Coverage-guided fuzzing
        |
 crash / hang / anomaly
        |
 Reproducible testcase (.mmf)
```

## Status

Early. The workspace currently provides the shared vocabulary — addresses,
access descriptors, a memory map and the `Bus` trait — and is growing from
there. See `docs/ARCHITECTURE.md` for the intended design.

## License

MIT. See `LICENSE`.
