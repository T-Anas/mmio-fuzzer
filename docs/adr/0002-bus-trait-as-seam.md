# 2. The Bus trait is the backend seam

* Status: accepted
* Date: recorded during development

## Context

Execution backends and peripheral models both need to see memory traffic, but they must not depend on each other.

## Decision

Define `Bus` and `AccessObserver` in `mmio-core`. `mmio-emu` implements `Bus`; the fuzzer attaches an observer.

## Consequences

Backends are interchangeable and the inference layer never imports the emulator.
