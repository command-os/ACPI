//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicAddrOverride {
    header: super::IcHeader,
    _reserved: u16,
    pub local_apic_addr: u64,
}

impl core::ops::Deref for LocalApicAddrOverride {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
