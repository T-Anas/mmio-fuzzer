# 6. The trace is an AccessObserver

* Status: accepted
* Date: recorded during development

## Context

Recording accesses should not require changing the bus or the core.

## Decision

Implement `AccessObserver for AccessLog` so a journal can be attached to any bus.

## Consequences

Tracing is composable: tests attach counters, the engine attaches a journal.
