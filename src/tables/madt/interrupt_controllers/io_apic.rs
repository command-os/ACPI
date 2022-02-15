/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IoApic {
    header: super::IcHeader,
    pub io_apic_id: u8,
    _reserved: u8,
    pub io_apic_address: u32,
    pub gsi_base: u32,
}

impl core::ops::Deref for IoApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
