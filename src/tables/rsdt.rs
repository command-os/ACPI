//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#[repr(C, packed)]
pub struct Rsdt {
    header: super::SdtHeader,
}

impl core::ops::Deref for Rsdt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for Rsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rsdt")
            .field("header", &self.header)
            .finish_non_exhaustive()
    }
}
