# 5. Inference lives in its own crate

* Status: accepted
* Date: recorded during development

## Context

Register inference is the intellectual centre and will grow independently of execution and fuzzing.

## Decision

Put trace types, the register model and the inference pass in `mmio-infer`, depending only on `mmio-core`.

## Consequences

The heuristics can be tested against synthetic traces with no emulator in the loop.
