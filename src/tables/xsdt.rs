//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#[repr(C, packed)]
pub struct Xsdt {
    header: super::SdtHeader,
}

impl core::ops::Deref for Xsdt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for Xsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Xsdt")
            .field("header", &self.header)
            .finish_non_exhaustive()
    }
}
