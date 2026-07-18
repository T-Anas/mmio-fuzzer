# 18. Prefer unknown to wrong

* Status: accepted
* Date: recorded during development

## Context

An over-confident model makes the fuzzer explore impossible device behaviour.

## Decision

Inference only commits to a claim when the trace supports it; otherwise the register stays generic.

## Consequences

The model is smaller but trustworthy, which matters more for a fuzzing aid.
