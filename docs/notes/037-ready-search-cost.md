# Cost of the ready-value search

Each polled register costs up to the candidate cap in extra executions.

With single bits first, the common case is cheap: bit zero or bit one unblocks the loop within a couple of tries. The cap bounds the worst case.
