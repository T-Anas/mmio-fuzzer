# What the loader handles

ELF and raw binary. We read program headers, not sections, because that is what gets loaded.

Symbols are not needed yet. If crash triage wants function names, the loader can grow a symbol table later.
