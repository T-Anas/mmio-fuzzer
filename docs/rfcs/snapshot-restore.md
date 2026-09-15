# Snapshot and restore

* Status: proposed


## Motivation

Re-profiling on every replay is wasteful.

## Proposal

Propose serialising the machine state so a run can resume from a checkpoint.

## Open questions

How to validate this without regressing current behaviour?
