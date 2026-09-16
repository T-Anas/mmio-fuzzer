# Interrupt injection

* Status: proposed


## Motivation

Timers and NVIC are not modelled actively.

## Proposal

Propose scheduling exceptions from the input so interrupt handlers become fuzzing surface.

## Open questions

How to validate this without regressing current behaviour?
