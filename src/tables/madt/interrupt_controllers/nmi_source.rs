/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

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
