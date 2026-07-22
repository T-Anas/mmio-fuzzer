# 13. Testcases are JSON with a .mmf extension

* Status: accepted
* Date: recorded during development

## Context

Findings must be inspectable, diffable and self-describing.

## Decision

Serialise firmware hash, input, finding and final PC as JSON with a `.mmf` suffix.

## Consequences

Humans can read them, tools can parse them, and replay verifies the firmware hash.
