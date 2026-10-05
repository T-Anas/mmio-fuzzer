# mmio-fuzz

An **MMIO-aware fuzzer for bare-metal ARM Cortex-M firmware**. It emulates a
real firmware image, watches the memory-mapped I/O it performs, infers a model
of the peripheral registers, and then fuzzes the responses to those reads to
drive the firmware down paths it would take on real silicon.

No board. No full datasheet.

```
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

## Why

Emulate firmware without peripheral models and every device read returns
`0x0`. Firmware gets stuck in `while (!(REG & READY)) {}`, or takes paths the
hardware would never allow, and coverage goes nowhere.

`mmio-fuzz` observes the MMIO traffic the firmware actually performs and builds
a *lightweight* hardware model from it. Reads are then answered from that
model, so the firmware keeps running — and the fuzzer can reach the code you
care about.

The interesting part is the inference: the tool learns that bit 0 of a status
register means "ready" by watching the firmware wait for it, and by searching
for the value that unblocks the wait.

## Quickstart

Rust 1.75+ is the only requirement to build and test; the firmware fixtures are
committed as ELF.

```sh
cargo build --workspace
cargo test --workspace
```

Run the bundled demo fixture:

```sh
cargo run -p mmio-cli -- run fixtures/prebuilt/uart_demo.elf \
    --iterations 2000 --out ./out
```

This profiles the firmware, infers its peripheral model, fuzzes it, and writes
`out/model.json` plus any findings as `out/*.mmf`.

Inspect the model:

```sh
cargo run -p mmio-cli -- inspect out/model.json
```

Reproduce a finding:

```sh
cargo run -p mmio-cli -- replay out/unmapped-*.mmf fixtures/prebuilt/uart_demo.elf
```

## What inference finds

For the demo fixture it reports, among other things:

```
mmio@0x40000000
  0x40000000  WO  0 r/1 w  width=u32
  0x40000004  RO  3333 r/0 w  width=u32  [const=0x00000001 polled ready=0x00000001]
  0x40000008  WO  0 r/1 w  width=u32
  0x4000000c  RO  1 r/0 w  width=u32
```

`0x4000_0004` is a read-only status register the firmware polls; the tool
discovered that reading `1` lets it proceed. It then found the fixture's seeded
memory-safety bug and wrote a replayable testcase.

## Architecture

A Cargo workspace with a strict dependency direction:

| Crate | Role |
| --- | --- |
| `mmio-core` | addresses, accesses, bus trait, memory map |
| `mmio-emu` | pure-Rust ARMv6-M interpreter, ELF loader, MMIO hooks |
| `mmio-infer` | access journal, register model, inference |
| `mmio-fuzz` | coverage, mutation, peripheral model, engine, testcases |
| `mmio-cli` | the `mmio-fuzz` binary |
| `xtask` | fixture builds and repository automation |

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and the design note on
[MMIO inference](docs/design/mmio-inference.md).

## Status

Working end to end on ARMv6-M fixtures: the acceptance test infers the ready
bit and finds a seeded bug. ARMv7-M, interrupt injection and richer peripheral
state machines are future work — see [docs/ROADMAP.md](docs/ROADMAP.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: `cargo fmt`, `cargo clippy
-- -D warnings` and `cargo test` must pass.

## License

MIT. See [LICENSE](LICENSE).
