# Writing a fixture

Keep it small and link it at the flash base with a vector table at the start.
Use `-mcpu=cortex-m0` so the compiler emits ARMv6-M only.
Expose the behaviour you want to test through MMIO registers.
