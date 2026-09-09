# Triaging a HardFault

The core took exception 3. The finding records the PC at the fault.
Check whether the fault was deliberate (a `bkpt` or `svc`) or the result of a bad branch or pointer.
Replay with the saved input to confirm, then disassemble around the PC.
