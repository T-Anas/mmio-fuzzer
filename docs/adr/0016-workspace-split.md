# 16. Split the tool into crates

* Status: accepted
* Date: recorded during development

## Context

A single crate would allow accidental coupling and slow incremental builds.

## Decision

Use a workspace: core, emu, infer, fuzz and cli, with a strict downward dependency direction.

## Consequences

Clear ownership and faster iteration; the cost is a little more boilerplate.
