# MMIO dependency graph

* Status: proposed


## Motivation

Registers influence each other through firmware control flow.

## Proposal

Propose inferring a graph of write-then-read dependencies between registers.

## Open questions

How to validate this without regressing current behaviour?
