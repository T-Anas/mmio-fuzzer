# Write-one-to-clear is only a hint

Writing one to an interrupt-status register clears a bit; separately, writing a command and reading a status can look identical.

We mark the pattern as a hint rather than a fact. The model serialiser prints `w1c?` so a human can judge.
