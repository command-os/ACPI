//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

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
