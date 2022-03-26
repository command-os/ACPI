//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

#[derive(Debug, BitfieldSpecifier, Clone, Copy)]
#[bits = 2]
pub enum BgrtOrientation {
    None = 0,
    Orient90,
    Orient180,
    Orient270,
}

#[bitfield(bits = 8)]
#[derive(Debug, Clone, Copy)]
pub struct BgrtStatus {
    pub displayed: bool,
    pub offset: BgrtOrientation,
    #[skip]
    __: B5,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Bgrt {
    header: super::SdtHeader,
    __: u16,
    pub status: BgrtStatus,
    ___: u8,
    pub image_addr: u64,
    pub image_off: (u32, u32),
}

impl core::ops::Deref for Bgrt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
