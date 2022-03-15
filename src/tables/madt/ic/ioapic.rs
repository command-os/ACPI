//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IoApic {
    header: super::IcHeader,
    pub id: u8,
    _reserved: u8,
    pub address: u32,
    pub gsi_base: u32,
}

impl core::ops::Deref for IoApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum IoApicRegister {
    IoApicId,
    IoApicVer,
    IoApicArbId,
    IoRedirTbl = 0x10,
}

#[derive(Debug, BitfieldSpecifier)]
#[repr(u8)]
#[bits = 3]
pub enum DeliveryMode {
    Fixed,
    LowestPriority,
    SMI,
    NMI = 4,
    INIT,
    ExtINT,
}

#[bitfield(bits = 64)]
#[derive(Debug)]
pub struct IoApicRedirect {
    pub vector: u8,
    pub delivery_mode: DeliveryMode,
    pub logical_dest: bool,
    pub pending: bool,
    pub active_high: bool,
    pub remote_irr: bool,
    pub trigger_at_level: bool,
    pub masked: bool,
    #[skip]
    __: B40,
    pub dest: B7,
}

impl IoApic {
    fn base(&self, off: u8) -> *mut u32 {
        (self.address as usize + off as usize + amd64::paging::PHYS_VIRT_OFFSET) as *mut u32
    }

    pub fn read(&self, reg: u32) -> u32 {
        unsafe {
            self.base(0).write_volatile(reg);
            self.base(0x10).read_volatile()
        }
    }

    pub fn write(&self, reg: u32, val: u32) {
        unsafe {
            self.base(0).write_volatile(reg);
            self.base(0x10).write_volatile(val);
        }
    }

    pub fn read_redir(&self, num: u32) -> IoApicRedirect {
        let base = IoApicRegister::IoRedirTbl as u32 + num * 2;
        IoApicRedirect::from_bytes(
            [
                self.read(base).to_le_bytes(),
                self.read(base + 1).to_le_bytes(),
            ]
            .concat()
            .as_slice()
            .try_into()
            .unwrap(),
        )
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Iso {
    header: super::IcHeader,
    pub bus: u8,
    pub irq: u8,
    pub gsi: u32,
    pub flags: amd64::spec::mps::Inti,
}

impl core::ops::Deref for Iso {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NmiSource {
    header: super::IcHeader,
    pub flags: amd64::spec::mps::Inti,
    pub gsi: u32,
}

impl core::ops::Deref for NmiSource {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
