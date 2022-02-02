/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IoApic {
    header: super::IcHeader,
    io_apic_id: u8,
    reserved: u8,
    io_apic_address: u32,
    gsi_base: u32,
}

impl IoApic {
    pub fn io_apic_id(&self) -> u8 {
        self.io_apic_id
    }

    pub fn io_apic_address(&self) -> u32 {
        self.io_apic_address
    }

    pub fn gsi_base(&self) -> u32 {
        self.gsi_base
    }
}

impl core::ops::Deref for IoApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
