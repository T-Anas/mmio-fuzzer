//! The inference pass itself.
//!
//! We walk the trace once to accumulate per-address statistics, then run a
//! handful of cheap pattern detectors:
//!
//! * **constant reads** — every read returned the same value;
//! * **poll loops** — the firmware reads the same address from the same PC
//!   many times in a row, which is how it waits for a status bit;
//! * **read side effects** — consecutive reads without an intervening write
//!   where bits get cleared, the signature of a read-to-clear register;
//! * **write side effects** — a write followed by a read where written bits
//!   come back zero, a hint of write-one-to-clear.

use std::collections::BTreeMap;

use mmio_core::AccessKind;

use crate::log::AccessLog;
use crate::model::HardwareModel;
use crate::register::RegisterModel;

/// Tuning knobs for the inference pass.
#[derive(Clone, Debug)]
pub struct InferConfig {
    /// How many consecutive reads from one PC count as a poll loop.
    pub poll_threshold: usize,
    /// Enable read/write side-effect heuristics.
    pub detect_side_effects: bool,
}

impl Default for InferConfig {
    fn default() -> Self {
        Self {
            poll_threshold: 4,
            detect_side_effects: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Read { value: u32, pc: u32 },
    Write { value: u32 },
}

/// Infers a [`HardwareModel`] from an access trace.
pub fn infer(log: &AccessLog, config: &InferConfig) -> HardwareModel {
    let mut stats: BTreeMap<u32, RegisterModel> = BTreeMap::new();
    let mut events: BTreeMap<u32, Vec<Event>> = BTreeMap::new();

    for access in log.mmio() {
        let addr = access.addr.raw();
        let value = access.value & access.width.mask();
        let reg = stats
            .entry(addr)
            .or_insert_with(|| RegisterModel::new(addr));
        reg.widen_to(access.width);

        let log_for_addr = events.entry(addr).or_default();
        match access.kind {
            AccessKind::Write => {
                reg.writes += 1;
                reg.write_values.insert(value);
                log_for_addr.push(Event::Write { value });
            }
            _ => {
                reg.reads += 1;
                reg.read_values.insert(value);
                log_for_addr.push(Event::Read {
                    value,
                    pc: access.pc,
                });
            }
        }
    }

    for (addr, evs) in &events {
        if let Some(reg) = stats.get_mut(addr) {
            analyse(reg, evs, config);
        }
    }

    HardwareModel::from_registers(stats.into_values().collect())
}

fn analyse(reg: &mut RegisterModel, events: &[Event], config: &InferConfig) {
    analyse_constness(reg);
    analyse_polls(reg, events, config);
    if config.detect_side_effects {
        analyse_read_side_effects(reg, events);
        analyse_write_side_effects(reg, events);
    }
}

fn analyse_constness(reg: &mut RegisterModel) {
    // Require more than one sample: a single read says nothing about constancy.
    if reg.reads >= 2 && reg.read_values.len() == 1 {
        reg.constant_read = reg.read_values.iter().next().copied();
    }
}

fn analyse_polls(reg: &mut RegisterModel, events: &[Event], config: &InferConfig) {
    let mut i = 0;
    // Remember the longest run so the ready value reflects the best evidence.
    let mut best: Option<(u32, usize, u32, u32)> = None; // (pc, len, first, last)
    while i < events.len() {
        if let Event::Read { pc, .. } = events[i] {
            let mut j = i;
            while j < events.len() {
                match events[j] {
                    Event::Read { pc: p, .. } if p == pc => j += 1,
                    _ => break,
                }
            }
            let run = &events[i..j];
            if run.len() >= config.poll_threshold {
                let first = match run[0] {
                    Event::Read { value, .. } => value,
                    _ => unreachable!(),
                };
                let last = match run[run.len() - 1] {
                    Event::Read { value, .. } => value,
                    _ => unreachable!(),
                };
                reg.polled = true;
                reg.poll_sites.insert(pc);
                if best.map(|(_, len, _, _)| run.len() > len).unwrap_or(true) {
                    best = Some((pc, run.len(), first, last));
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }

    if let Some((_, _, first, last)) = best {
        reg.ready_value = Some(last);
        reg.ready_mask = first ^ last;
    }
}

/// Detects registers whose read clears bits: two reads without an intervening
/// write where the later value has fewer set bits.
fn analyse_read_side_effects(reg: &mut RegisterModel, events: &[Event]) {
    let mut previous: Option<u32> = None;
    for event in events {
        match *event {
            Event::Write { .. } => previous = None,
            Event::Read { value, .. } => {
                if let Some(prev) = previous {
                    if prev != value && (prev & !value) != 0 {
                        reg.read_to_clear = true;
                    }
                }
                previous = Some(value);
            }
        }
    }
}

/// Detects the write-one-to-clear signature: a write followed by a read in
/// which some bit that was written as 1 comes back as 0.
fn analyse_write_side_effects(reg: &mut RegisterModel, events: &[Event]) {
    let mut last_write: Option<u32> = None;
    for event in events {
        match *event {
            Event::Write { value } => last_write = Some(value),
            Event::Read { value, .. } => {
                if let Some(written) = last_write.take() {
                    if written != 0 && (written & !value) != 0 {
                        reg.write_to_clear_hint = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::AccessLog;
    use mmio_core::{Access, AccessWidth, PhysAddr};

    fn read(addr: u32, value: u32, pc: u32, seq: u64) -> Access {
        Access::new(
            AccessKind::Read,
            PhysAddr::new(addr),
            AccessWidth::Word,
            value,
            pc,
            seq,
        )
    }

    fn write(addr: u32, value: u32, seq: u64) -> Access {
        Access::new(
            AccessKind::Write,
            PhysAddr::new(addr),
            AccessWidth::Word,
            value,
            0,
            seq,
        )
    }

    fn run(accesses: Vec<Access>) -> HardwareModel {
        let log = AccessLog::from_vec(accesses);
        infer(&log, &InferConfig::default())
    }

    #[test]
    fn detects_constant_read() {
        let model = run(vec![
            read(0x4000_0000, 0x1234_5678, 0x100, 0),
            read(0x4000_0000, 0x1234_5678, 0x100, 1),
        ]);
        assert_eq!(
            model.register(0x4000_0000).unwrap().constant_read,
            Some(0x1234_5678)
        );
    }

    #[test]
    fn detects_poll_loop_and_ready_value() {
        let mut accesses = Vec::new();
        for (i, v) in [0u32, 0, 0, 0, 1].iter().enumerate() {
            accesses.push(read(0x4000_0008, *v, 0x200, i as u64));
        }
        let model = run(accesses);
        let reg = model.register(0x4000_0008).unwrap();
        assert!(reg.polled);
        assert_eq!(reg.ready_value, Some(1));
        assert_eq!(reg.ready_mask, 1);
        // The ready value is a strong suggestion for unblocking the firmware.
        assert_eq!(model.suggest_read(0x4000_0008, 0), 1);
    }

    #[test]
    fn detects_read_to_clear() {
        let model = run(vec![
            read(0x4000_0010, 0x1, 0x300, 0),
            read(0x4000_0010, 0x0, 0x300, 1),
        ]);
        assert!(model.register(0x4000_0010).unwrap().read_to_clear);
    }

    #[test]
    fn detects_write_to_clear_hint() {
        let model = run(vec![
            write(0x4000_0014, 0x10, 0),
            read(0x4000_0014, 0x00, 0x400, 1),
        ]);
        assert!(model.register(0x4000_0014).unwrap().write_to_clear_hint);
    }

    #[test]
    fn ignores_non_peripheral_addresses() {
        let model = run(vec![
            read(0x2000_0000, 0x41, 0x100, 0),
            write(0x2000_0004, 0x42, 1),
        ]);
        assert_eq!(model.register_count(), 0);
    }
}
