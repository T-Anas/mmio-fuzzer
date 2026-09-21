# Frequently asked questions

**Does this need the real hardware?**

No. The whole point is to fuzz firmware without the board.

**Do I need a datasheet?**

No. The model is inferred from the firmware's own accesses. A datasheet would
help you interpret the result, not run the tool.

**Do I need a cross compiler?**

No. The bundled fixtures are committed as ELF. You only need one to build your
own firmware or rebuild the fixtures.

**Why not just use Unicorn or QEMU?**

Both are excellent. We chose a small pure-Rust core for total control over MMIO
interception and to avoid native dependencies. A Unicorn backend is a proposed
RFC, not a rejection.

**Why does my firmware hang immediately?**

It is probably polling a status register the model has not learned yet. Check
the model report for a `polled` register; if the ready value is missing, raise
`max_discovery_candidates`.

**Why are some plausible values wrong?**

Inference is a heuristic. It tells you how a register behaves in the traces it
saw, not what it means. The value list is deliberately small.

**Is a hang a real bug?**

Sometimes. A firmware that ends in an idle loop is reported as a hang too. Use
the PC in the finding to tell them apart.

**Can it fuzz ARMv7-M firmware?**

Not yet. ARMv6-M first. The architecture is documented and the extension path
is clear.

**How do I reproduce a finding?**

`mmio-fuzz replay case.mmf firmware.elf`. The testcase records the firmware
hash and the exact input.
