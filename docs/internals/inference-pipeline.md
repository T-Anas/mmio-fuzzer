# The inference pipeline, step by step

`infer(log, config)` walks the trace once, then analyses each address.

1. **Collect.** For every peripheral access, look up or create the register
   model for its address. Widen the modelled width, count the access, record
   the masked value, and append an event (`Read { value, pc }` or
   `Write { value }`) to a per-address stream.
2. **Constancy.** If the register was read at least twice and every read
   returned the same value, record that value as constant.
3. **Polls.** Scan the event stream for runs of `Read` with the same PC. A run
   at least `poll_threshold` long marks the register polled and records the
   PC. The longest run supplies the ready value (last read) and the ready mask
   (first XOR last).
4. **Read side effects.** Walk events tracking the last read since the last
   write. A later read with fewer bits set marks read-to-clear.
5. **Write side effects.** Walk events tracking the last write. A read in which
   a written 1 came back 0 marks the write-one-to-clear hint.

The result is grouped into 4 KiB blocks and returned as a `HardwareModel`.
