# NVIC is not modelled yet

Same story as SysTick: reads and writes to the NVIC are captured as MMIO.

Real interrupt delivery is future work. For the current fixtures, cooperative polling is enough.
