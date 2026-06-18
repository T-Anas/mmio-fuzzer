# Inferring device behaviour from MMIO traces

The hard part of firmware fuzzing without hardware is that the firmware and
the device are in a dialogue, and we only hear one side. This document explains
what `mmio-infer` listens for and why.

## What we get

Every bus transaction the core performs is recorded, in order, with:

* the address and transfer width;
* whether it was a read or a write;
* the value (written, or returned);
* the program counter that produced it;
* a monotonic sequence number.

That is enough to reconstruct a surprising amount.

## Registers

Transactions are grouped by exact address. A group is a register. Its width is
the widest transfer observed; a peripheral addressed with `u8` accesses in one
place and `u32` in another is described by the `u32` view.

## Access style

Reads and writes are counted. Only reads means a status or ID register. Only
writes means a command or configuration register. Both means a control/status
pair.

## Constant reads

If a register is read more than once and always returns the same value, we
treat it as constant. This restores ID registers and fixed configuration
values, which is often what firmware checks before it does anything useful.

One read is not evidence: a single sample could be anything, so we require at
least two.

## Poll loops

Firmware waits for hardware with a loop of the shape:

```c
while ((STATUS & READY) == 0) { }
```

Compiled for Cortex-M0 this becomes `ldr`/`tst`/`beq` with a fixed PC for the
load. We detect a poll loop as a run of reads from the *same PC to the same
address* longer than a threshold. The address is a status register, and the
run is the wait.

The value that ends the loop is the interesting one. In the trace collected
with no device model the loop never ends, so we search for the ready value:

1. try each single bit `1 << n`;
2. then all ones;
3. then any value the firmware was seen to read.

The first candidate that lets the firmware reach code the baseline never
reached is accepted. That is how the tool learns that bit 0 means "ready".

## Side effects

* **Read-to-clear**: two reads with no intervening write where the second has
  bits cleared that the first had set. Typical of interrupt status registers.
* **Write-one-to-clear** (hint): a write followed by a read in which a bit
  written as 1 comes back 0. Marked as a hint because command/status register
  pairs produce a similar shape.

## Limits

This is inference, not reverse engineering. It cannot tell you what a register
means, only how it behaves. It will mis-classify unusual registers. The model
is a fuzzing aid, and the value list it produces is deliberately small: a
handful of plausible values beats a flood of random ones when the goal is
coverage.
