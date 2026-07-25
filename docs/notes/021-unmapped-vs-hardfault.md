# Unmapped access or HardFault?

A bad pointer store is caught by the bus as an unmapped access before any exception is taken.

The engine reports whichever comes first. Both are target bugs; the distinction is useful when triaging.
