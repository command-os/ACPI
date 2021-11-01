/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicAddrOverride {
    header: super::IcHeader,
    reserved: u16,
    local_apic_addr: u64,
}

impl LocalApicAddrOverride {
    pub fn local_apic_addr(&self) -> u64 {
        self.local_apic_addr
    }
}

impl core::ops::Deref for LocalApicAddrOverride {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
