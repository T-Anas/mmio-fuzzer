# Anomaly kinds

HardFault, invalid opcode, unmapped access, hang and emulator error.

The first three are target bugs. Invalid opcode is usually a tooling gap and emulator error is always our fault, so `is_target_bug` separates them for reporting.
