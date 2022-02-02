/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use core::any::type_name;

#[repr(C, packed)]
pub struct NmiSource {
    header: super::IcHeader,
    flags: amd64::spec::mps::Inti,
    gsi: u32,
}

impl NmiSource {
    pub fn flags(&self) -> amd64::spec::mps::Inti {
        self.flags
    }

    pub fn gsi(&self) -> u32 {
        self.gsi
    }
}

impl core::ops::Deref for NmiSource {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for NmiSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("header", &self.header)
            .field("flags", &self.flags())
            .field("gsi", &self.gsi())
            .finish()
    }
}
