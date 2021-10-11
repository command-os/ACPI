mod bgrt;
mod rsdp;
mod rsdt;
mod xsdt;

pub use bgrt::*;
pub use rsdp::*;
pub use rsdt::*;
pub use xsdt::*;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SDTHeader {
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

impl SDTHeader {
    pub fn validate(&self) -> bool {
        let bytes =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, self.length()) };
        let sum = bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte));

        sum == 0
    }

    pub fn signature(&self) -> &[u8; 4] {
        &self.signature
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }

    pub fn oem_id(&self) -> &[u8; 6] {
        &self.oem_id
    }

    pub fn oem_table_id(&self) -> &[u8; 8] {
        &self.oem_table_id
    }

    pub fn oem_revision(&self) -> u32 {
        self.oem_revision
    }

    pub fn creator_id(&self) -> &[u8; 4] {
        &self.creator_id
    }

    pub fn creator_revision(&self) -> u32 {
        self.creator_revision
    }
}
