# Fuzzing a parser

This guide shows how to point `mmio-fuzz` at a real parsing library compiled
for a Cortex-M0 target, using MQTT-C as the worked example.

## 1. Build the library and a harness

A parser target is a normal firmware image: a vector table, a `main` that runs
the parser on a buffer, and a way to signal completion. The harness lives in
`targets/mqtt-c/harness`:

* `main.c` calls `mqtt_unpack_response` on the input buffer, then plays the
  part of an application (copying a payload, walking return codes);
* `compat.c` provides the handful of libc routines the library needs;
* `mqtt_pal_min.h` is a minimal platform layer;
* `startup.s` / `link.ld` lay out flash and RAM.

Build it:

```sh
export ARM_GCC=/path/to/arm-none-eabi-gcc
cargo xtask build-targets
```

## 2. Describe the memory layout

Give the input its own region and leave holes around the buffers:

```sh
--input-mem 0x20004000:0x1000 \
--ram   0x20000000:0x1000 \   # .data/.bss
--stack 0x20008000:0x1000 \
--heap  0x20002000:0x1000 \
--guard 0x20001000:0x1000 \
--guard 0x20003000:0x1000 \
--guard 0x20005000:0x3000
```

`--guard` marks the holes whose hits are reported as `GuardHit` rather than a
generic unmapped access. The holes are unmapped by construction: any address
not listed in a region is unmapped.

## 3. Provide seeds

A seed is a valid packet; the fuzzer mutates from it. One file per packet type:

```sh
--seed-file seed/publish.bin --seed-file seed/suback.bin ...
```

The engine biases peripheral reads and, for buffer targets, mutates the bytes
loaded into `--input-mem`.

## 4. Run and replay

```sh
mmio-fuzz run target.elf --iterations 300000 ... --out out/
mmio-fuzz replay out/guard-*.mmf target.elf
```

Findings are `.mmf` files that carry the firmware hash, the input and the
machine layout, so replay reconstructs the exact machine.

## 5. Use a negative control

Build the library with the suspected bug fixed and confirm the finding
disappears. That is the difference between a real bug and a harness artefact.
`cargo xtask build-targets` produces `mqtt_publish_fixed.elf` for exactly this
purpose.

## Tips

* Keep guards immediately after every buffer a parser writes into or reads
  from; a guard *before* the buffer catches underflows too.
* Signal completion through a halt register (`--halt-addr`) so successful
  parses are not reported as hangs.
* Prefer several small seeds over one large one; coverage grows faster.
* If everything faults immediately, check that `.bss`/`.data` are mapped.
