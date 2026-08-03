# Coverage hashing

The bucket index is `(pc >> 1) ^ prev`, masked to 64 KiB, with `prev` tracking the previous edge.

Shifting PC right by one folds the Thumb bit into the same bucket as the instruction address.
