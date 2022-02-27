//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#[derive(Debug)]
pub enum RsdtType {
    Rsdt(&'static super::Rsdt),
    Xsdt(&'static super::Xsdt),
}

#[repr(C, packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
    length: u32,
    xsdt_addr: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl Rsdp {
    pub fn validate(&self) -> bool {
        let bytes =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, self.length()) };
        let sum = bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte));

        sum == 0
    }

    pub fn signature(&self) -> &str {
        core::str::from_utf8(&self.signature).unwrap().trim()
    }

    pub fn oem_id(&self) -> &str {
        core::str::from_utf8(&self.oem_id).unwrap().trim()
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }

    /// If ACPI 1.0, return fixed size, else return length field
    pub fn length(&self) -> usize {
        match self.revision {
            0 => 20,
            _ => self.length as usize,
        }
    }

    pub fn into_type(&self) -> RsdtType {
        // This is fine.
        unsafe {
            match self.revision {
                0 => RsdtType::Rsdt((self.rsdt_addr as *const super::Rsdt).as_ref().unwrap()),
                _ => RsdtType::Xsdt((self.xsdt_addr as *const super::Xsdt).as_ref().unwrap()),
            }
        }
    }
}

impl core::fmt::Debug for Rsdp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rsdp")
            .field("valid", &self.validate())
            .field("oem_id", &self.oem_id())
            .field("revision", &self.revision())
            .field("length", &self.length())
            .field("type", &self.into_type())
            .finish()
    }
}
