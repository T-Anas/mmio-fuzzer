# Parallel fuzzing

* Status: proposed


## Motivation

One execution at a time limits throughput.

## Proposal

Propose sharing the corpus and model across worker processes with periodic synchronisation.

## Open questions

How to validate this without regressing current behaviour?
