/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Iso {
    header: super::IcHeader,
    pub bus: u8,
    pub source: u8,
    pub gsi: u32,
    pub flags: amd64::spec::mps::Inti,
}

impl core::ops::Deref for Iso {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
