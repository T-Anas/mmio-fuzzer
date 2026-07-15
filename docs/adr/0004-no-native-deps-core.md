# 4. Keep the core free of native dependencies

* Status: accepted
* Date: recorded during development

## Context

The toolchain should build on any developer machine and in CI without system packages.

## Decision

Only `mmio-emu` and below are dependency-light; the cross toolchain is needed solely to rebuild fixtures, which are committed prebuilt.

## Consequences

`cargo test` works out of the box. Fixture regeneration is opt-in via xtask.
