# 20. Deduplicate findings by kind and PC

* Status: accepted
* Date: recorded during development

## Context

Havoc fuzzing rediscovers the same fault thousands of times.

## Decision

Record at most one finding per (anomaly kind, final PC) pair, up to a cap.

## Consequences

Reports stay readable; the corpus still keeps every new-coverage input.
