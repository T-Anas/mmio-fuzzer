# Replay re-runs profiling

A testcase stores the input, not the model, because profiling is deterministic for a given firmware.

Replay rebuilds the model from scratch and checks the same anomaly appears. If it does not, the finding was flaky and should not be trusted.
