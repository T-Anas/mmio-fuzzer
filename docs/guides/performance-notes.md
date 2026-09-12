# Performance notes

The interpreter is unoptimised and runs tens of thousands of instructions per second in debug.
Release builds are dramatically faster; use them for long campaigns.
Profiling one polled register costs a bounded number of extra executions.
