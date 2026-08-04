# The input is opaque on purpose

The engine never interprets input bytes as a structure.

They exist only to bias value selection. Keeping them opaque means mutation needs no grammar and no format-specific code.
