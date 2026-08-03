# Classifying faults

`CoreError::is_target_fault` returns true for unmapped, misaligned and invalid opcode.

Adding the method in `mmio-core` keeps the classification next to the errors rather than duplicated in the engine.
