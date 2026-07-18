# 1. Use a pure-Rust ARMv6-M interpreter

* Status: accepted
* Date: recorded during development

## Context

Unicorn and QEMU can execute Cortex-M firmware, but both need native libraries and neither gives us first-class control over MMIO interception. The core value of this project is in how peripheral accesses are observed and modelled.

## Decision

Implement the ARMv6-M instruction set ourselves behind a `Bus` trait, and keep a Unicorn backend possible via the same trait.

## Consequences

We own the hot loop and can record every access cheaply. The cost is a large amount of instruction-level work and a risk of subtle decoding bugs.
