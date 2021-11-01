/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Bgrt {
    header: super::SdtHeader,
    /// Must be 1
    version: u16,
    /// bits 2:1 orientation offset; 0b00 = no offset, 0b01 = 90, 0b10 = 180, 0b11 = 270
    status: u8,
    /// Must be 0
    image_type: u8,
    image_addr: u64,
    image_off: (u32, u32),
}

impl Bgrt {
    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn status(&self) -> u8 {
        self.status
    }

    pub fn image_type(&self) -> u8 {
        self.image_type
    }

    pub fn image_addr(&self) -> u64 {
        self.image_addr
    }

    pub fn image_off(&self) -> (u32, u32) {
        self.image_off
    }
}

impl core::ops::Deref for Bgrt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
