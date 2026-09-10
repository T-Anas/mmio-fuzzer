# Triaging an unmapped access

The bus refused an address nothing maps, before any exception.
This is our earliest signal of a memory-safety bug and the easiest to reproduce.
Look at the access kind and address in the finding detail; a store to a computed address is the common case.
