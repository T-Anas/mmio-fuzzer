//! A flat, map-backed bus with MMIO hooks and an access observer.
//!
//! `FlatMemory` is the default [`Bus`] used by the interpreter. RAM and flash
//! are stored as little-endian byte vectors; MMIO regions are forwarded to a
//! pluggable [`MmioHandler`], which is where the fuzzer's peripheral model
//! lives. Every transaction is offered to an [`AccessObserver`] before it is
//! returned to the core.

use std::collections::HashMap;

use crate::shadow::Shadow;
use mmio_core::{
    Access, AccessKind, AccessObserver, AccessWidth, Bus, CoreError, MemoryMap, MemoryRegion,
    PhysAddr, RegionKind, Result,
};

/// Decides how an MMIO region behaves.
///
/// Returning `None` means "no opinion", in which case reads yield 0 and writes
/// are dropped. Implementations can be as simple as a register array or as
/// elaborate as a scripted state machine.
pub trait MmioHandler {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Option<u32>;
    fn write(&mut self, addr: PhysAddr, width: AccessWidth, value: u32) -> Option<()>;
}

/// An [`MmioHandler`] that always returns zero and swallows writes.
#[derive(Default, Debug, Clone, Copy)]
pub struct NullMmio;

impl MmioHandler for NullMmio {
    fn read(&mut self, _addr: PhysAddr, _width: AccessWidth) -> Option<u32> {
        Some(0)
    }

    fn write(&mut self, _addr: PhysAddr, _width: AccessWidth, _value: u32) -> Option<()> {
        Some(())
    }
}

#[inline]
fn read_le(data: &[u8], offset: u32, width: AccessWidth) -> u32 {
    let start = offset as usize;
    let n = width.bytes() as usize;
    let mut value = 0u32;
    for i in 0..n {
        if let Some(byte) = data.get(start + i) {
            value |= (*byte as u32) << (8 * i);
        }
    }
    value
}

#[inline]
fn write_le(data: &mut [u8], offset: u32, width: AccessWidth, value: u32) {
    let start = offset as usize;
    let n = width.bytes() as usize;
    for i in 0..n {
        if start + i < data.len() {
            data[start + i] = (value >> (8 * i)) as u8;
        }
    }
}

/// A simple bus that keeps RAM and flash in host memory.
pub struct FlatMemory {
    map: MemoryMap,
    data: HashMap<String, Vec<u8>>,
    mmio: Box<dyn MmioHandler>,
    observer: Option<Box<dyn AccessObserver>>,
    seq: u64,
    /// PC supplied by the core before each access, for observers that care.
    current_pc: u32,
    /// Optional shadow memory for redzone checks.
    shadow: Option<Shadow>,
    /// When true, touching an unmapped address is an error instead of a zero.
    strict: bool,
}

impl FlatMemory {
    pub fn new(map: MemoryMap) -> Self {
        let mut data = HashMap::new();
        for region in map.iter() {
            if !region.kind.is_mmio() && region.size > 0 {
                data.insert(region.name.clone(), vec![0u8; region.size as usize]);
            }
        }
        Self {
            map,
            data,
            mmio: Box::new(NullMmio),
            observer: None,
            seq: 0,
            current_pc: 0,
            shadow: None,
            strict: true,
        }
    }

    /// Maps a zeroed region after construction.
    pub fn map_region(&mut self, region: MemoryRegion) -> Result<()> {
        if !region.kind.is_mmio() && region.size > 0 {
            self.data
                .insert(region.name.clone(), vec![0u8; region.size as usize]);
        }
        self.map
            .insert(region)
            .map_err(|e| CoreError::Bus(e.to_string()))
    }

    /// Replaces the MMIO handler.
    pub fn set_mmio_handler(&mut self, handler: impl MmioHandler + 'static) {
        self.mmio = Box::new(handler);
    }

    /// Registers an observer that sees every access.
    pub fn set_observer(&mut self, observer: impl AccessObserver + 'static) {
        self.observer = Some(Box::new(observer));
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    /// Records the PC of the instruction currently being executed so that
    /// observers can attribute accesses to a code location.
    pub fn set_current_pc(&mut self, pc: u32) {
        self.current_pc = pc;
    }

    /// Installs a shadow-memory configuration.
    pub fn set_shadow(&mut self, shadow: Shadow) {
        self.shadow = Some(shadow);
    }

    pub fn shadow(&self) -> Option<&Shadow> {
        self.shadow.as_ref()
    }

    /// Poisons `[base, base+size)` as a redzone, creating the shadow if needed.
    pub fn poison(&mut self, base: u32, size: u32) {
        self.shadow
            .get_or_insert_with(Shadow::new)
            .poison(base, size);
    }

    pub fn map(&self) -> &MemoryMap {
        &self.map
    }

    /// Copies an image into whichever region contains `base`.
    ///
    /// This models reset-time flash aliasing: firmware is loaded at `base` and
    /// the region's backing store is filled byte for byte.
    pub fn load_image(&mut self, base: PhysAddr, bytes: &[u8]) -> Result<()> {
        let name = self
            .map
            .region_of(base)
            .ok_or(CoreError::Unmapped {
                addr: base,
                kind: AccessKind::Write,
                width: AccessWidth::Byte,
            })?
            .to_string();
        let region_base = self.map.find(base).expect("region exists").base;
        let offset = (base - region_base) as usize;
        let data = self.data.get_mut(&name).ok_or(CoreError::Bus(format!(
            "region {name} has no backing store"
        )))?;
        if offset + bytes.len() > data.len() {
            return Err(CoreError::Bus(format!(
                "image of {} bytes does not fit in region {name}",
                bytes.len()
            )));
        }
        data[offset..offset + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    /// Reads without observing, for host-side inspection.
    pub fn peek(&self, addr: PhysAddr, width: AccessWidth) -> Option<u32> {
        let region = self.map.find(addr)?;
        let off = region.offset(addr)?;
        if region.kind.is_mmio() {
            return None;
        }
        self.data.get(&region.name).map(|d| read_le(d, off, width))
    }

    fn record(
        &mut self,
        kind: AccessKind,
        addr: PhysAddr,
        width: AccessWidth,
        value: u32,
        pc: u32,
    ) {
        self.seq += 1;
        let pc = if pc != 0 { pc } else { self.current_pc };
        let access = Access::new(kind, addr, width, value, pc, self.seq);
        if let Some(observer) = self.observer.as_mut() {
            observer.observe(&access);
        }
    }
}

impl Bus for FlatMemory {
    fn read(&mut self, addr: PhysAddr, width: AccessWidth) -> Result<u32> {
        if let Some(shadow) = &self.shadow {
            shadow.check(addr, AccessKind::Read, width)?;
        }
        let region = self.map.find(addr).cloned();
        let value = match region {
            Some(region) => {
                let off = region.offset(addr).unwrap_or(0);
                if region.kind.is_mmio() {
                    self.mmio.read(addr, width).unwrap_or(0)
                } else {
                    match self.data.get(&region.name) {
                        Some(d) => read_le(d, off, width),
                        None => 0,
                    }
                }
            }
            None => {
                if self.strict {
                    return Err(CoreError::Unmapped {
                        addr,
                        kind: AccessKind::Read,
                        width,
                    });
                }
                0
            }
        };
        // Observers see the value actually returned to the core.
        self.record(AccessKind::Read, addr, width, value, 0);
        Ok(value)
    }

    fn write(&mut self, addr: PhysAddr, width: AccessWidth, value: u32) -> Result<()> {
        if let Some(shadow) = &self.shadow {
            shadow.check(addr, AccessKind::Write, width)?;
        }
        let region = self.map.find(addr).cloned();
        match region {
            Some(region) => {
                let off = region.offset(addr).unwrap_or(0);
                if region.kind.is_mmio() {
                    self.mmio.write(addr, width, value);
                } else if let Some(d) = self.data.get_mut(&region.name) {
                    write_le(d, off, width, value);
                }
            }
            None => {
                if self.strict {
                    return Err(CoreError::Unmapped {
                        addr,
                        kind: AccessKind::Write,
                        width,
                    });
                }
            }
        }
        self.record(AccessKind::Write, addr, width, value, 0);
        Ok(())
    }

    fn set_current_pc(&mut self, pc: u32) {
        self.current_pc = pc;
    }
}

/// The PC that produced the most recent access, when a core supplies it.
///
/// The bus itself has no PC, but observers that need it can combine the raw
/// access with the core's current PC. Kept as a helper to document intent.
pub fn is_peripheral(map: &MemoryMap, addr: PhysAddr) -> bool {
    map.is_mmio(addr)
}

/// Region kinds for which a backing store is allocated.
pub fn needs_backing(kind: RegionKind) -> bool {
    !kind.is_mmio()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mmio_core::{Permissions, RegionKind};

    fn memory() -> FlatMemory {
        let mut map = MemoryMap::new();
        map.insert(MemoryRegion::new(
            "flash",
            PhysAddr::new(0x0800_0000),
            0x1000,
            RegionKind::Flash,
            Permissions::RX,
        ))
        .unwrap();
        map.insert(MemoryRegion::new(
            "sram",
            PhysAddr::new(0x2000_0000),
            0x1000,
            RegionKind::Ram,
            Permissions::RW,
        ))
        .unwrap();
        map.insert(MemoryRegion::new(
            "uart0",
            PhysAddr::new(0x4000_0000),
            0x1000,
            RegionKind::Peripheral,
            Permissions::RW,
        ))
        .unwrap();
        FlatMemory::new(map)
    }

    #[test]
    fn reads_and_writes_ram_little_endian() {
        let mut m = memory();
        m.write(PhysAddr::new(0x2000_0000), AccessWidth::Word, 0xdead_beef)
            .unwrap();
        assert_eq!(
            m.peek(PhysAddr::new(0x2000_0000), AccessWidth::Byte),
            Some(0xef)
        );
        assert_eq!(
            m.read(PhysAddr::new(0x2000_0000), AccessWidth::Word)
                .unwrap(),
            0xdead_beef
        );
    }

    #[test]
    fn mmio_defaults_to_zero_and_swallows_writes() {
        let mut m = memory();
        assert_eq!(
            m.read(PhysAddr::new(0x4000_0004), AccessWidth::Word)
                .unwrap(),
            0
        );
        m.write(PhysAddr::new(0x4000_0004), AccessWidth::Word, 0x1234)
            .unwrap();
    }

    #[test]
    fn strict_mode_rejects_unmapped() {
        let mut m = memory();
        let err = m.read(PhysAddr::new(0x5000_0000), AccessWidth::Word);
        assert!(matches!(err, Err(CoreError::Unmapped { .. })));
        m.set_strict(false);
        assert_eq!(
            m.read(PhysAddr::new(0x5000_0000), AccessWidth::Word)
                .unwrap(),
            0
        );
    }

    #[test]
    fn observer_sees_every_access() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let mut m = memory();
        m.set_observer(mmio_core::FnObserver(move |a: &Access| {
            sink.borrow_mut().push(a.addr.raw())
        }));
        m.write(PhysAddr::new(0x2000_0000), AccessWidth::Word, 1)
            .unwrap();
        m.read(PhysAddr::new(0x4000_0000), AccessWidth::Word)
            .unwrap();
        assert_eq!(&*seen.borrow(), &[0x2000_0000, 0x4000_0000]);
    }

    #[test]
    fn load_image_places_bytes() {
        let mut m = memory();
        m.load_image(PhysAddr::new(0x0800_0000), &[1, 2, 3, 4])
            .unwrap();
        assert_eq!(
            m.read(PhysAddr::new(0x0800_0000), AccessWidth::Word)
                .unwrap(),
            0x0403_0201
        );
    }

    #[test]
    fn redzone_access_is_out_of_bounds() {
        let mut m = memory();
        m.poison(0x2000_0100, 0x10);
        assert!(matches!(
            m.read(PhysAddr::new(0x2000_00ff), AccessWidth::HalfWord),
            Err(CoreError::OutOfBounds { .. })
        ));
        assert!(matches!(
            m.write(PhysAddr::new(0x2000_0104), AccessWidth::Word, 1),
            Err(CoreError::OutOfBounds { .. })
        ));
        assert!(m
            .read(PhysAddr::new(0x2000_0000), AccessWidth::Word)
            .is_ok());
    }
}
