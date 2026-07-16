# 8. The fuzz input is the external world

* Status: accepted
* Date: recorded during development

## Context

A bare-metal firmware has no argv or stdin. Its inputs are peripheral responses.

## Decision

Treat the fuzz input as an opaque seed that biases which plausible peripheral value is returned.

## Consequences

Coverage feedback directly measures how well we are modelling the device.
