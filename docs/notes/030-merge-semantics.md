# Merging observations

Profiling runs the firmware several times and merges the models.

Read and write counts add, value sets union, and boolean flags OR together. Constancy is recomputed after the merge so a register that was constant in one run but not another is not mislabelled.
