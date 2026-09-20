# Glossary

**Access observer** — a sink for bus transactions. `AccessLog` is one; the
bus calls `observe` for every access.

**Anomaly** — a finding: a fault, a hang, or a tooling error. See
`docs/triage`.

**ARMv6-M** — the architecture of the Cortex-M0/M0+. Thumb only, roughly fifty
instructions.

**Block** — a 4 KiB group of registers in the hardware model, named after its
base address.

**Bus** — the trait through which the core reaches memory and MMIO. The
backend seam.

**Coverage map** — an AFL-style bitmap of PC transitions used to guide
mutation.

**Corpus** — the inputs that produced new coverage.

**FlatMemory** — the default `Bus`: RAM and flash in host memory, MMIO
delegated to a handler.

**Hardware model** — the inferred description of peripheral registers.

**MMIO** — memory-mapped I/O: device registers reached through ordinary loads
and stores.

**PeripheralModel** — the MMIO handler that answers reads from the inferred
model, biased by the fuzz input.

**Poll loop** — firmware spinning on a status register until a bit reads set.

**Ready value** — the value that ends a poll loop, found by search.

**Read-to-clear** — a register whose read clears the returned bits.

**Register model** — what is known about a single address: width, values,
access style and side effects.

**Testcase (.mmf)** — a reproducible JSON record of a finding.

**Write-one-to-clear** — a register where writing a 1 clears the corresponding
bit.
