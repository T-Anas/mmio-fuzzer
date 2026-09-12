# Peripheral state machines

* Status: proposed


## Motivation

The register model is mostly stateless.

## Proposal

Propose a small FSM per peripheral learned from access sequences so writes change later reads.

## Open questions

How to validate this without regressing current behaviour?
