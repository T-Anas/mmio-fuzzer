# Future: peripheral state machines

The current model is per-register and stateless apart from read-to-clear.

A natural extension is a small state machine per peripheral learned from access sequences, so writes change subsequent reads in richer ways. The register model is designed to be a component of that.
