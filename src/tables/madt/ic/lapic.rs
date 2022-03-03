//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

#[bitfield(bits = 32)]
#[repr(C, u32)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicFlags {
    #[skip(setters)]
    pub enabled: bool,
    #[skip(setters)]
    pub online_capable: bool,
    #[skip]
    __: B30,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApic {
    header: super::IcHeader,
    pub acpi_uid: u8,
    pub apic_id: u8,
    pub flags: LocalApicFlags,
}

impl core::ops::Deref for LocalApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicAddrOverride {
    header: super::IcHeader,
    _reserved: u16,
    pub addr: u64,
}

impl core::ops::Deref for LocalApicAddrOverride {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicNmi {
    header: super::IcHeader,
    pub acpi_proc_id: u8,
    pub flags: amd64::spec::mps::Inti,
    pub lint: u8,
}

impl core::ops::Deref for LocalApicNmi {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
