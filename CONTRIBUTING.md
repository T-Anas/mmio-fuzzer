# Contributing

Thanks for looking. A few rules keep this project pleasant to work on.

## Building

```sh
cargo build --workspace
cargo test --workspace
```

The test suite is self-contained: the firmware fixtures are committed as ELF
so you do not need a cross toolchain to run it.

## Rebuilding fixtures

If you change anything under `fixtures/src`, rebuild the committed images:

```sh
APT=?? # install gcc-arm-none-eabi, or point ARM_GCC at one
export ARM_GCC=/path/to/arm-none-eabi-gcc
cargo xtask build-fixtures
```

Commit both the source change and the regenerated `fixtures/prebuilt/*.elf`.

## Style

* `cargo fmt` and `cargo clippy -- -D warnings` must pass.
* Keep the dependency direction intact: lower crates never depend on higher
  ones. `mmio-core` depends on nothing in the workspace.
* Prefer a small amount of honest code over a clever abstraction. The emulator
  follows the architecture manual; when it departs, say why in a comment.
* Every non-trivial behaviour gets a test. Tests use assembled bytes or
  synthetic access traces, not the network.

## Commits

Small and focused. One idea per commit, imperative subject, a line of body when
the *why* is not obvious from the *what*.
