# 17. Use a seeded ChaCha8 RNG

* Status: accepted
* Date: recorded during development

## Context

Findings are only useful if runs reproduce.

## Decision

Seed all mutation from an explicit engine seed with ChaCha8, and record the seed and input.

## Consequences

Runs replay exactly on the same build.
