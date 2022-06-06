//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::mem::size_of;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MCFG {
    header: super::SDTHeader,
    __: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MCFGEntry {
    pub base: u64,
    pub segment: u32,
    pub bus_start: u8,
    pub bus_end: u8,
    __: u32,
}

impl MCFG {
    pub fn entries(&self) -> &'static [MCFGEntry] {
        let len = (self.length() - size_of::<Self>()) / size_of::<MCFGEntry>();
        unsafe {
            let data = (self as *const _ as *const u8).add(size_of::<Self>()) as *const MCFGEntry;
            core::slice::from_raw_parts(data, len)
        }
    }
}

impl core::ops::Deref for MCFG {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
