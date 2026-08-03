# Profiling is deterministic

During profiling, registers with no model return zero rather than a hashed value.

That makes the baseline trace a stable description of what the firmware does with a dead bus, which is what the inference heuristics assume.
