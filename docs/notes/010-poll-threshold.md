# Choosing the poll threshold

A poll loop must be seen at least four times from the same PC to count.

Four is low enough to catch short waits and high enough that a couple of unconditional reads do not look like polling. It is a config field, not a constant buried in code.
