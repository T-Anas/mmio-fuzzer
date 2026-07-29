# Hangs versus normal termination

A firmware that ends in `for (;;)` looks like a hang to a step-budget engine.

We detect stagnation by counting instructions since the last new edge. A tight loop reaches the limit quickly; the resulting finding is labelled with its PC so a human can see it is just the idle loop.
