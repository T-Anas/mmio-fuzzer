# Reset reads the vector table

`CortexM::reset` loads SP from address zero and the reset vector from address four.

Because the vector table is aliased there, firmware linked at the flash base resets correctly with no special casing in the core.
