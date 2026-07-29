# Deduplicating findings

Havoc fuzzing finds the same fault again and again with different inputs.

We keep one finding per anomaly kind and PC. The corpus still stores every new-coverage input, so coverage progress is not lost.
