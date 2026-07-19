# Little-endian loading

`FlatMemory::load_image` copies bytes verbatim; multi-byte reads assemble little-endian.

This matches ARM's default and means an ELF segment can be copied straight in without swapping. Tests assert the byte order explicitly so a future big-endian target cannot silently break it.
