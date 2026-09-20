# Corpus and coverage

Coverage is a 64 KiB byte array of saturating counters indexed by
`(pc >> 1) ^ previous_pc`, both folded into 16 bits. Recording an edge that was
zero increments the edge count; merging ORs another map in and reports how many
edges were new.

The corpus is a list of inputs. An input joins the corpus when its coverage map
contains an edge the global map has never seen. Selection for mutation is a
simple round-robin over the corpus indexed by the execution count, which keeps
the engine deterministic.

Deduplication of corpus entries is by coverage digest rather than by input
bytes: two different inputs that reach the same edges add nothing.
