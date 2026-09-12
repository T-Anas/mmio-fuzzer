# The replay workflow

Findings are written as `.mmf` files next to the model.
`mmio-fuzz replay case.mmf firmware.elf` rebuilds the model and re-checks the finding.
A finding that does not reproduce should be discarded as flaky.
