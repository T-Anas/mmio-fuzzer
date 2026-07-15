# 7. Use an AFL-style edge bitmap

* Status: accepted
* Date: recorded during development

## Context

We need a cheap notion of 'new behaviour' to guide mutation.

## Decision

Hash the transition between consecutive PCs into a 64 KiB bitmap with saturating counters.

## Consequences

O(1) per instruction, trivially mergeable, and good enough to tell progress from stagnation.
