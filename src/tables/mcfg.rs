//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::mem::size_of;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Mcfg {
    header: super::SdtHeader,
    __: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct McfgEntry {
    pub base: u64,
    pub segment: u32,
    pub bus_start: u8,
    pub bus_end: u8,
    __: u32,
}

impl Mcfg {
    pub fn entries(&self) -> &'static [McfgEntry] {
        let len = (self.length() - size_of::<Self>()) / size_of::<McfgEntry>();
        unsafe {
            let data = (self as *const _ as *const u8).add(size_of::<Self>()) as *const McfgEntry;
            core::slice::from_raw_parts(data, len)
        }
    }
}

impl core::ops::Deref for Mcfg {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
