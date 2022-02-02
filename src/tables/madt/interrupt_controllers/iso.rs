/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Iso {
    header: super::IcHeader,
    bus: u8,
    source: u8,
    gsi: u32,
    flags: amd64::spec::mps::Inti,
}

impl Iso {
    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn source(&self) -> u8 {
        self.source
    }

    pub fn gsi(&self) -> u32 {
        self.gsi
    }

    pub fn flags(&self) -> amd64::spec::mps::Inti {
        self.flags
    }
}

impl core::ops::Deref for Iso {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
