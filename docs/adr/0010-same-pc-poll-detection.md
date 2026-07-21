# 10. Detect polls by repeated reads from one PC

* Status: accepted
* Date: recorded during development

## Context

Waiting for hardware compiles to `ldr`/`tst`/`beq` with a fixed load PC.

## Decision

Flag an address as polled when the same PC reads it more than a threshold number of times in a run.

## Consequences

We identify status registers robustly without symbolic execution.
