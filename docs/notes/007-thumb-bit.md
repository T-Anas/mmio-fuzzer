# The Thumb bit is always set

There is no ARM state on Cortex-M, so EPSR.T is hard-wired to one.

`Xpsr::from_bits` forces it, and branches mask bit zero of their target. This keeps exception return values and BLX honest.
