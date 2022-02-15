/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Bgrt {
    pub header: super::SdtHeader,
    /// Must be 1
    pub version: u16,
    /// bits 2:1 orientation offset; 0b00 = no offset, 0b01 = 90, 0b10 = 180, 0b11 = 270
    pub status: u8,
    /// Must be 0
    pub image_type: u8,
    pub image_addr: u64,
    pub image_off: (u32, u32),
}

impl core::ops::Deref for Bgrt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
