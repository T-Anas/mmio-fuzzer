# 3. Target ARMv6-M before ARMv7-M

* Status: accepted
* Date: recorded during development

## Context

ARMv6-M is roughly fifty instructions and no coprocessor, while ARMv7-M is an order of magnitude larger.

## Decision

Implement ARMv6-M faithfully and extend toward Thumb-2 only when a fixture needs it.

## Consequences

A trustworthy small core now, with an honest extension path, beats a broad untrustworthy one.
