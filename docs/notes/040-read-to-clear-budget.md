# Modelling read-to-clear

For a register flagged read-to-clear, the first read returns a useful value and subsequent reads return zero until a write.

That is a crude but effective approximation that stops a firmware from spinning on a register whose bits it is itself consuming.
