# 14. Commit prebuilt firmware images

* Status: accepted
* Date: recorded during development

## Context

CI and contributors should not need a cross compiler to run tests.

## Decision

Commit the ELF images under `fixtures/prebuilt` and keep the build reproducible via xtask.

## Consequences

Tests run everywhere; the cost is binary artefacts in the repository.
