# Coverage measures the device model

When the fuzz input only changes which plausible value a register returns, every new edge is evidence that the current device model opened a path.

This is a nicer signal than in memory-corruption fuzzing: progress directly corresponds to understanding the peripheral.
