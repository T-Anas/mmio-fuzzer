# Architecture

`mmio-fuzz` is a Cargo workspace of small crates with a strict dependency
direction. Nothing lower in the stack knows about anything higher.

```
            +-------------------+
            |     mmio-cli      |  `mmio-fuzz` binary: run / infer / replay
            +---------+---------+
                      |
            +---------v---------+
            |     mmio-fuzz     |  engine, coverage, mutation, testcases
            +----+---------+----+
                 |         |
     +-----------v--+   +--v-----------+
     |  mmio-infer  |   |   mmio-emu   |  inference  /  ARMv6-M interpreter
     +------+-------+   +------+-------+
            |                  |
            +--------+---------+
                     |
             +-------v--------+
             |   mmio-core    |  addresses, accesses, bus trait, memory map
             +----------------+
```

## `mmio-core`

The vocabulary. `PhysAddr`, `Access`, `AccessKind`, `AccessWidth`, `Bus`,
`AccessObserver`, `MemoryMap`, `MemoryRegion`, `CoreError`. This crate has no
dependency on the others and compiles in microseconds.

## `mmio-emu`

A pure-Rust interpreter for ARMv6-M, the architecture of the Cortex-M0/M0+. It
is deliberately free of native dependencies so the whole tool builds and runs
anywhere Rust does.

* `regs` — general registers, xPSR, CONTROL, MSP/PSP.
* `decode` — Thumb decoding, split into a 16-bit and a 32-bit pass.
* `exec` — instruction semantics, exception entry, reset.
* `memory` — a `Bus` backed by a `MemoryMap` with an MMIO handler hook and an
  access observer.
* `loader` — ELF and raw image loading.

The `Bus` trait is the seam: `mmio-emu` never assumes MMIO lives in this
process. A future Unicorn backend implements the same trait.

## `mmio-infer`

Turns an `AccessLog` into a `HardwareModel`. See
[`design/mmio-inference.md`](design/mmio-inference.md) for the heuristics.

## `mmio-fuzz`

The engine. It builds a `Machine` for a firmware image, runs it under a
`PeripheralModel` (inference + fuzz input), records coverage, and keeps inputs
that reach new edges or trigger anomalies. Findings are serialised as
reproducible `.mmf` testcases.

## Data flow

```
firmware.elf ──▶ Machine ──▶ CortexM ──▶ Bus ──▶ PeripheralModel
                    │                         │
                    │                         └─ ▶ AccessLog ──▶ mmio-infer
                    │                                              │
                    └──────────────◀── HardwareModel ◀────────────┘
                                          │
                                    fuzz input (bias)
                                          │
                                     findings (.mmf)
```
