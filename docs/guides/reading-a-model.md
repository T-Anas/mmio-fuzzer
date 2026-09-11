# Reading an inferred model

Each line is one register: address, access style, counts, width and notes.
`const`, `polled`, `ready`, `r2c` and `w1c?` are the semantics the engine acts on.
Use `mmio-fuzz inspect model.json` to print a saved model.
