# ADC and SBC carry handling

ADCS adds the carry flag; SBCS subtracts one minus the carry flag.

Getting SBC's borrow arithmetic right mattered: the intermediate is computed as a 64-bit difference so the carry-out is `!borrow`, which is what the architecture specifies.
