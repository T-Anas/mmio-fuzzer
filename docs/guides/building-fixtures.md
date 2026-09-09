# Building firmware fixtures

Install `gcc-arm-none-eabi` and point `ARM_GCC` at it.
Run `cargo xtask build-fixtures`.
Commit both the source and the regenerated ELF.
