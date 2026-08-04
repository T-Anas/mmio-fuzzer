# Stack pointer alignment

Reset masks SP down to an 8-byte boundary, as the architecture requires.

PUSH and POP use word granularity and rely on the base being aligned for doubleword correctness.
