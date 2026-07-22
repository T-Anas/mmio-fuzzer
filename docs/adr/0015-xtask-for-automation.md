# 15. Repository automation goes in xtask

* Status: accepted
* Date: recorded during development

## Context

Fixture builds and future checks need a home that is not a fragile shell script.

## Decision

Add an `xtask` crate invoked as `cargo xtask`.

## Consequences

Automation is written in Rust, type-checked, and versioned with the code.
