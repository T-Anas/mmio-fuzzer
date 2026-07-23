# 19. A shared error taxonomy in mmio-core

* Status: accepted
* Date: recorded during development

## Context

Errors cross crate boundaries and need a consistent vocabulary.

## Decision

Define `CoreError` with variants for unmapped, misaligned, invalid opcode, step limit and backend gaps.

## Consequences

The fuzzer can distinguish target faults from tooling limitations with `is_target_fault`.
