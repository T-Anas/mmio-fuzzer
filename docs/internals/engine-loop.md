# The engine loop

`Engine::run` does two phases.

**Profiling.** Execute with an empty model to capture a baseline trace. Infer a
model. Execute again with that model to capture more traffic. For each polled
register, search for a ready value and record it as an override. Execute a
third time with the overrides, then merge everything into the final model.

**Fuzzing.** For each iteration: pick a corpus entry, mutate it, execute under
the model and overrides, merge any new coverage, and record target-bug
findings. Findings are deduplicated by anomaly kind and PC.

Each execution builds a fresh bus so no state leaks between runs. The observer
is an `Rc<RefCell<AccessLog>>` shared with the engine; after the core is
dropped the log can be taken out without cloning.
