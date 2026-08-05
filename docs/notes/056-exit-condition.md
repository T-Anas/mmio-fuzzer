# When a run ends

A run ends on an exception, a bus error, the step budget or coverage stagnation.

There is no 'program exited' notion because bare-metal firmware has none; the idle loop is the natural end state and is reported as a hang.
