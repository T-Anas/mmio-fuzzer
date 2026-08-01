# Interesting mutation words

The mutator occasionally writes words like zero, all ones, 0x8000_0000 and 0x4000_0000.

These are the values that most often flip a branch or an address computation, so they are worth inserting even in a byte-oriented mutator.
