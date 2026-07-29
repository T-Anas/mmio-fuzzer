# Why ChaCha8

Mutation speed matters less than reproducibility, but ChaCha8 is fast and has a tiny state.

The engine owns one seeded generator. No global RNG is used anywhere, so two engines with the same seed produce the same corpus.
