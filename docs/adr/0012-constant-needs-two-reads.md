# 12. Require two reads before calling a register constant

* Status: accepted
* Date: recorded during development

## Context

A single read sample is not evidence of constancy and caused spurious constants.

## Decision

Only mark a register constant when it has been read at least twice with identical results.

## Consequences

Fewer false constants; registers with one observation stay fuzzable.
