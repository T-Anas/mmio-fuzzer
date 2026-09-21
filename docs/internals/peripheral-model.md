# The peripheral model

`PeripheralModel` implements `MmioHandler`. On a read it does, in order:

1. return an override if one is set for the address;
2. return the constant value if the register is known constant;
3. select one of the register's plausible values, biased by the input and a
   per-address counter;
4. return the fallback for unknown registers.

Read-to-clear registers then consume a one-shot budget: the first useful read
succeeds and subsequent reads return zero until a write re-arms the register.

Selection hashes the address, the counter and the input bytes, reducing the
digest to an index into the value list. Mutating the input therefore changes
which plausible value is returned, which is exactly what coverage feedback
rewards.
