# PC latching during execution

Reading PC in a data-processing instruction yields the instruction address plus four.

We keep `r[15]` on the current instruction while it executes and expose `pc_relative()`. After execution the step function advances PC unless the instruction branched.
