//! The peripheral model used during emulation.
//!
//! This is where the inferred [`HardwareModel`] meets the `Bus`: reads to a
//! known register are answered from the register's list of plausible values,
//! biased by the fuzz input so mutations steer device interactions.

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use mmio_core::{AccessWidth, PhysAddr};
use mmio_emu::memory::MmioHandler;
use mmio_infer::HardwareModel;

use crate::input::Input;

/// Shared cell the halt device writes and the engine polls to end a run.
pub type ExitCell = Rc<Cell<Option<u32>>>;

/// A byte stream delivered through a simple UART-like register bank.
///
/// Firmware drivers typically poll a status register for "receive ready" and
/// then read bytes from a data register. Modelling that over MMIO is the
/// canonical way to fuzz a parser that consumes untrusted input.
struct Stream {
    base: u32,
    pos: usize,
    data: Vec<u8>,
    /// Width of one chunk read from the data register.
    width: AccessWidth,
}

impl Stream {
    const DATA: u32 = 0x00;
    const STATUS: u32 = 0x04;
    const COUNT: u32 = 0x0c;
    const RX_READY: u32 = 0x1;
    const TX_READY: u32 = 0x2;
    const SPAN: u32 = 0x1000;

    fn contains(&self, addr: u32) -> bool {
        addr >= self.base && addr < self.base.wrapping_add(Self::SPAN)
    }

    fn read(&mut self, offset: u32, access: AccessWidth) -> u32 {
        let value = match offset {
            Self::DATA => {
                let chunk = self.width.bytes() as usize;
                let mut value = 0u32;
                for i in 0..chunk {
                    let byte = self.data.get(self.pos + i).copied().unwrap_or(0);
                    value |= (byte as u32) << (8 * i);
                }
                if self.pos < self.data.len() {
                    self.pos = (self.pos + chunk).min(self.data.len());
                }
                value
            }
            Self::STATUS => {
                let mut status = Self::TX_READY;
                if self.pos < self.data.len() {
                    status |= Self::RX_READY;
                }
                status
            }
            Self::COUNT => (self.data.len() - self.pos) as u32,
            _ => 0,
        };
        value & access.mask()
    }

    fn write(&mut self, _offset: u32, _value: u32) {
        // A real device would transmit; the model has nothing to do.
    }
}

/// Halt register: writing a non-zero exit code ends the run cleanly.
struct Halt {
    addr: u32,
    cell: ExitCell,
}

/// An MMIO handler driven by an inferred model and a fuzz input.
pub struct PeripheralModel {
    model: HardwareModel,
    input: Input,
    /// Forced read values, used during value discovery and for known-ready
    /// status registers.
    overrides: HashMap<u32, u32>,
    /// Per-address read counter, so repeated reads can return different
    /// plausible values.
    counters: HashMap<u32, u32>,
    /// Remaining reads that should return zero for read-to-clear registers.
    clear_budget: HashMap<u32, u32>,
    /// Optional streaming input device.
    stream: Option<Stream>,
    /// Optional halt register shared with the engine.
    halt: Option<Halt>,
    /// Value returned for registers with no model at all.
    pub fallback: u32,
    /// When false, the model is ignored and `fallback` is returned always.
    pub use_model: bool,
}

impl PeripheralModel {
    pub fn new(model: HardwareModel, input: Input) -> Self {
        Self {
            model,
            input,
            overrides: HashMap::new(),
            counters: HashMap::new(),
            clear_budget: HashMap::new(),
            stream: None,
            halt: None,
            fallback: 0,
            use_model: true,
        }
    }

    pub fn with_overrides(mut self, overrides: HashMap<u32, u32>) -> Self {
        self.overrides = overrides;
        self
    }

    pub fn with_fallback(mut self, fallback: u32) -> Self {
        self.fallback = fallback;
        self
    }

    /// Installs a UART-like device that streams `data` to the firmware.
    pub fn with_stream(self, base: u32, data: Vec<u8>) -> Self {
        self.with_stream_width(base, data, AccessWidth::Byte)
    }

    /// Installs a streaming device whose data register returns `width`-sized
    /// chunks. Modelling the chunk size matters for peripherals such as CAN
    /// mailboxes or word-wide FIFOs.
    pub fn with_stream_width(mut self, base: u32, data: Vec<u8>, width: AccessWidth) -> Self {
        self.stream = Some(Stream {
            base,
            pos: 0,
            data,
            width,
        });
        self
    }

    /// Installs a halt register. Writing it ends the run with that code.
    pub fn with_halt(mut self, addr: u32, cell: ExitCell) -> Self {
        self.halt = Some(Halt { addr, cell });
        self
    }

    /// Selects one value from `values` using the input and read counter.
    fn select(&self, addr: u32, counter: u32, values: &[u32]) -> u32 {
        if values.is_empty() {
            return self.fallback;
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(&addr.to_le_bytes());
        hasher.update(&counter.to_le_bytes());
        hasher.update(&self.input.bytes);
        let digest = hasher.finalize();
        let index = (u16::from_le_bytes([digest.as_bytes()[0], digest.as_bytes()[1]]) as usize)
            % values.len();
        values[index]
    }
}

impl MmioHandler for PeripheralModel {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Option<u32> {
        let a = addr.raw();

        // The streaming device owns its register bank outright.
        if let Some(stream) = self.stream.as_mut() {
            if stream.contains(a) {
                return Some(stream.read(a - stream.base, width));
            }
        }

        let counter = {
            let entry = self.counters.entry(a).or_insert(0);
            let current = *entry;
            *entry = entry.wrapping_add(1);
            current
        };

        if let Some(value) = self.overrides.get(&a).copied() {
            return Some(value & width.mask());
        }

        let value = if !self.use_model {
            self.fallback
        } else if let Some(constant) = self.model.register(a).and_then(|reg| reg.constant_read) {
            // A register that always reads the same value is not a fuzzing
            // surface: return it deterministically.
            constant
        } else {
            match self.model.register(a) {
                Some(reg) => {
                    let values = reg.plausible_reads();
                    self.select(a, counter, &values)
                }
                // Unknown registers read as zero during profiling so the
                // firmware's behaviour is deterministic and observable.
                None => self.fallback,
            }
        };

        // Read-to-clear: after one useful read, subsequent reads return zero
        // until the register is written again.
        if self.use_model {
            if let Some(reg) = self.model.register(a) {
                if reg.read_to_clear {
                    let budget = self.clear_budget.entry(a).or_insert(0);
                    if *budget > 0 {
                        *budget -= 1;
                        return Some(0);
                    }
                    *budget = 1;
                }
            }
        }

        Some(value & width.mask())
    }

    fn write(&mut self, addr: PhysAddr, _width: AccessWidth, value: u32) -> Option<()> {
        let a = addr.raw();

        // Halt register: the harness writes an exit code to end the run.
        if let Some(halt) = self.halt.as_ref() {
            if a == halt.addr {
                halt.cell.set(Some(value));
                return Some(());
            }
        }

        if let Some(stream) = self.stream.as_mut() {
            if stream.contains(a) {
                stream.write(a - stream.base, value);
                return Some(());
            }
        }

        // A write arms the next read-to-clear cycle again.
        self.clear_budget.remove(&a);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mmio_infer::register::RegisterModel;

    fn model_with(reg: RegisterModel) -> HardwareModel {
        HardwareModel::from_registers(vec![reg])
    }

    #[test]
    fn constant_register_is_returned() {
        let mut reg = RegisterModel::new(0x4000_0000);
        reg.reads = 1;
        reg.constant_read = Some(0xabcd);
        reg.width = AccessWidth::Word;
        let mut handler = PeripheralModel::new(model_with(reg), Input::new());
        let value = handler
            .read(PhysAddr::new(0x4000_0000), AccessWidth::Word)
            .unwrap();
        assert_eq!(value, 0xabcd);
    }

    #[test]
    fn overrides_take_priority() {
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new());
        let mut overrides = HashMap::new();
        overrides.insert(0x4000_0000u32, 0x55u32);
        handler = handler.with_overrides(overrides);
        assert_eq!(
            handler
                .read(PhysAddr::new(0x4000_0000), AccessWidth::Word)
                .unwrap(),
            0x55
        );
    }

    #[test]
    fn read_to_clear_returns_zero_after_first_read() {
        let mut reg = RegisterModel::new(0x4000_0000);
        reg.reads = 2;
        reg.read_to_clear = true;
        reg.constant_read = Some(0x1);
        reg.ready_value = Some(0x1);
        reg.width = AccessWidth::Word;
        let mut handler = PeripheralModel::new(model_with(reg), Input::new());
        let first = handler
            .read(PhysAddr::new(0x4000_0000), AccessWidth::Word)
            .unwrap();
        let second = handler
            .read(PhysAddr::new(0x4000_0000), AccessWidth::Word)
            .unwrap();
        assert_eq!(first, 0x1);
        assert_eq!(second, 0);
    }

    #[test]
    fn width_masks_the_value() {
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new());
        handler = handler.with_fallback(0xdead_beef);
        handler.use_model = false;
        assert_eq!(
            handler
                .read(PhysAddr::new(0x4000_0000), AccessWidth::Byte)
                .unwrap(),
            0xef
        );
    }

    #[test]
    fn stream_delivers_bytes_then_signals_empty() {
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new())
            .with_stream(0x4000_2000, vec![0xaa, 0xbb]);
        let status = |h: &mut PeripheralModel| {
            h.read(PhysAddr::new(0x4000_2004), AccessWidth::Word)
                .unwrap()
        };
        let data = |h: &mut PeripheralModel| {
            h.read(PhysAddr::new(0x4000_2000), AccessWidth::Word)
                .unwrap()
        };
        assert_eq!(status(&mut handler) & 1, 1, "rx ready");
        assert_eq!(data(&mut handler), 0xaa);
        assert_eq!(data(&mut handler), 0xbb);
        assert_eq!(status(&mut handler) & 1, 0, "stream drained");
        assert_eq!(data(&mut handler), 0);
    }

    #[test]
    fn halt_register_sets_the_exit_cell() {
        let exit: ExitCell = Rc::new(Cell::new(None));
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new())
            .with_halt(0x4000_f000, exit.clone());
        handler
            .write(PhysAddr::new(0x4000_f000), AccessWidth::Word, 7)
            .unwrap();
        assert_eq!(exit.get(), Some(7));
    }

    #[test]
    fn chunk_device_width_and_count() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut handler = PeripheralModel::new(HardwareModel::new(), Input::new())
            .with_stream_width(0x4000_3000, data, AccessWidth::Word);
        let read = |h: &mut PeripheralModel, off: u32| {
            h.read(PhysAddr::new(0x4000_3000 + off), AccessWidth::Word)
                .unwrap()
        };
        assert_eq!(read(&mut handler, 0x0c), 8, "count starts at eight");
        assert_eq!(read(&mut handler, 0x00), 0x0403_0201);
        assert_eq!(read(&mut handler, 0x0c), 4, "four bytes consumed");
        assert_eq!(read(&mut handler, 0x00), 0x0807_0605);
        assert_eq!(read(&mut handler, 0x0c), 0, "drained");
    }
}
