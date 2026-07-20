# The exception stack frame

On exception entry we push r0-r3, r12, lr, pc and xPSR, eight words, in ascending address order starting at SP-32.

This is the ARMv6-M frame layout. The minimal implementation does not yet handle tail-chaining or late arrival; single exceptions are enough for the fixtures we fuzz.
