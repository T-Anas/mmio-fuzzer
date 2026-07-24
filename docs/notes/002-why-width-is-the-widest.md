# Register width is the widest transfer

A peripheral can be touched byte-wise in one place and word-wise in another, usually by different driver layers.

We model the register at the widest width seen. Narrower accesses are treated as partial views and masked against that width when suggesting values.
