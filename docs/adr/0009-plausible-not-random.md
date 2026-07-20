# 9. Return plausible values, not random ones

* Status: accepted
* Date: recorded during development

## Context

Random MMIO responses send firmware down paths no real device would allow.

## Decision

Have the peripheral model select from the small set of values inference considers plausible for each register.

## Consequences

Coverage stays meaningful, and findings are far more likely to be real.
