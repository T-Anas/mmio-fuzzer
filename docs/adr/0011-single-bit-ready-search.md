# 11. Find ready values by single-bit search

* Status: accepted
* Date: recorded during development

## Context

With no device model a poll loop never ends, so we never observe the ready value.

## Decision

When a poll is detected, try each single bit, then all ones, then observed values, keeping the first that reaches new coverage.

## Consequences

The engine bootstraps itself past init code using nothing but coverage.
