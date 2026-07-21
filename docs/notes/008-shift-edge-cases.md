# Shift edge cases

LSL/LSR/ASR by zero leave flags untouched. LSL by 32 writes zero and sets C from bit zero of the source; ASR saturates the shift amount at 32.

These cases are easy to get wrong and are covered directly by unit tests.
