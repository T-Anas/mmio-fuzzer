# Roadmap

A rough, honest ordering of what would come next. Nothing here is a promise.

## Near term

- More ARMv6-M coverage: complete the hint and barrier encodings and add
  execution tests for every instruction, not just decoding.
- A second and third fixture: a read-to-clear protocol and a HardFault via a
  corrupted function pointer.
- Model diffing so two firmware revisions can be compared.

## Medium term

- Interrupt injection: schedule exceptions from the input so handlers become
  fuzzing surface.
- A simple DMA engine that moves bytes and raises completion bits.
- Peripheral state machines learned from access sequences.
- Persistent corpora and resumable campaigns.

## Long term

- An optional Unicorn backend for firmware beyond ARMv6-M.
- Parallel fuzzing across cores with a shared corpus.
- Symbol-aware reports that print function names next to PCs.

## Explicitly out of scope

- Reliable exploitation or exploitation primitives. This is a testing tool.
- Any dependency on a specific vendor's device tree.
