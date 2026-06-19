# Fixtures

Small Cortex-M0 firmware images used by the test suite and the acceptance
demo.

| Fixture | Purpose |
| --- | --- |
| `uart_demo` | Polls a status register, then branches on an MMIO command containing a seeded memory-safety bug. |

## Layout

```
fixtures/
  src/        C and assembly sources, linker script
  prebuilt/   committed ELF images (so tests need no cross toolchain)
  build/      intermediate objects (gitignored)
```

## Rebuilding

```sh
export ARM_GCC=/path/to/arm-none-eabi-gcc
cargo xtask build-fixtures
```

The images are linked at `0x0800_0000` (the usual Cortex-M flash base) with the
vector table at the start. Loading them, `mmio-fuzz` aliases the vector table
down to address zero, mirroring the hardware's boot alias.
