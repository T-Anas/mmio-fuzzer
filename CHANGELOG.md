# Changelog

All notable changes to this project are documented here. The format is loosely
based on Keep a Changelog; the project follows semantic versioning.

## [Unreleased]

### Added

- `mmio-core`: physical addresses, access descriptors, memory maps, the `Bus`
  trait and access observers.
- `mmio-emu`: a pure-Rust ARMv6-M interpreter with MMIO hooks, ELF loading and
  a minimal exception model.
- `mmio-infer`: register inference (width, constants, poll loops, read-to-clear
  and write-one-to-clear hints) and a JSON hardware model.
- `mmio-fuzz`: coverage-guided engine with a peripheral model, havoc mutator,
  corpus and reproducible `.mmf` testcases.
- `mmio-cli`: the `mmio-fuzz` binary with `run`, `infer`, `replay` and
  `inspect`.
- A buggy `uart_demo` Cortex-M0 fixture and an end-to-end acceptance test that
  infers its ready bit and finds its seeded memory-safety bug.

[Unreleased]: https://github.com/t-anas/mmio-fuzz/commits/main
