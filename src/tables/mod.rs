/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

pub use bgrt::*;
pub use madt::*;
pub use rsdp::*;
pub use rsdt::*;
pub use xsdt::*;

mod bgrt;
mod madt;
mod rsdp;
mod rsdt;
pub mod signatures;
mod xsdt;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: [u8; 4],
    creator_revision: u32,
}

impl SdtHeader {
    pub fn validate(&self) -> bool {
        let bytes =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, self.length()) };
        let sum = bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte));

        sum == 0
    }

    pub fn signature(&self) -> &str {
        core::str::from_utf8(&self.signature).unwrap().trim()
    }

    pub fn length(&self) -> usize {
        self.length.try_into().unwrap()
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }

    pub fn oem_id(&self) -> &str {
        core::str::from_utf8(&self.oem_id).unwrap().trim()
    }

    pub fn oem_table_id(&self) -> &str {
        core::str::from_utf8(&self.oem_table_id).unwrap().trim()
    }

    pub fn oem_revision(&self) -> u32 {
        self.oem_revision
    }

    pub fn creator_id(&self) -> &str {
        core::str::from_utf8(&self.creator_id).unwrap().trim()
    }

    pub fn creator_revision(&self) -> u32 {
        self.creator_revision
    }
}

impl core::fmt::Debug for SdtHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("valid", &self.validate())
            .field("signature", &self.signature())
            .field("length", &self.length())
            .field("revision", &self.revision())
            .field("oem_id", &self.oem_id())
            .field("oem_table_id", &self.oem_table_id())
            .field("oem_revision", &self.oem_revision())
            .field("creator_id", &self.creator_id())
            .field("creator_revision", &self.creator_revision())
            .finish()
    }
}
