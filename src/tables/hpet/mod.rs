//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

pub mod regs;

#[derive(Debug, BitfieldSpecifier, Clone, Copy)]
#[bits = 8]
#[repr(u8)]
pub enum AddressSpaceID {
    SystemMemory = 0,
    SystemIo,
}

#[bitfield(bits = 96)]
#[derive(Debug, Clone, Copy)]
pub struct Address {
    #[skip(setters)]
    pub addr_space_id: AddressSpaceID,
    #[skip(setters)]
    pub reg_bit_width: u8,
    #[skip(setters)]
    pub reg_bit_off: u8,
    #[skip]
    __: u8,
    #[skip(setters)]
    pub address: u64,
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct EventTimerBlockID {
    #[skip(setters)]
    pub hw_revision: u8,
    #[skip(setters)]
    pub comparator_cnt: B5,
    #[skip(setters)]
    pub counter_size: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub legacy_replacement: bool,
    #[skip(setters)]
    pub pci_vendor_id: u16,
}

#[derive(Debug, BitfieldSpecifier, Clone, Copy)]
#[bits = 4]
#[repr(u8)]
pub enum PageProtection {
    NoGuarantee = 0,
    Protected4Kb,
    Protected64Kb,
}

#[bitfield(bits = 8)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub struct PageProtectionAttributes {
    #[skip(setters)]
    pub page_protection: PageProtection,
    #[skip]
    __: B4,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct HPET {
    header: super::SDTHeader,
    pub evnt_timer_block: EventTimerBlockID,
    pub address: Address,
    pub hpet_num: u8,
    pub min_tick: u16,
    pub page_prot_attr: PageProtectionAttributes,
}

impl core::ops::Deref for HPET {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
