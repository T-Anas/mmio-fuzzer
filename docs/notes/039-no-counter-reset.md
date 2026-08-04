# Counters are not reset between reads

The peripheral model keeps a per-address read counter that only grows.

It feeds the value-selection hash together with the input, giving different reads within one execution different values, which is closer to real hardware.
