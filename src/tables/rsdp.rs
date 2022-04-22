//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::{any::type_name, mem::size_of};

use super::{rsdt::Rsdt, xsdt::Xsdt};

#[repr(C, packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    pub revision: u8,
    rsdt_addr: u32,
    length: u32,
    xsdt_addr: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

#[derive(Debug)]
pub enum RsdtType {
    Rsdt(&'static Rsdt),
    Xsdt(&'static Xsdt),
}

#[derive(Debug)]
pub struct RsdtTypeIter {
    ptr: *const u8,
    is_xsdt: bool,
    curr: usize,
    total: usize,
}

impl Iterator for RsdtTypeIter {
    type Item = &'static super::SdtHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.total {
            None
        } else {
            unsafe {
                let addr = if self.is_xsdt {
                    (self.ptr as *const u64).add(self.curr).read_volatile() as usize
                } else {
                    (self.ptr as *const u32).add(self.curr).read_volatile() as usize
                };
                self.curr += 1;
                ((addr + amd64::paging::PHYS_VIRT_OFFSET) as *const super::SdtHeader).as_ref()
            }
        }
    }
}

impl RsdtType {
    pub fn iter(&self) -> RsdtTypeIter {
        unsafe {
            let (is_xsdt, length, header) = match *self {
                RsdtType::Rsdt(v) => (false, v.length(), v as *const _ as *const u8),
                RsdtType::Xsdt(v) => (true, v.length(), v as *const _ as *const u8),
            };
            let total = (length - size_of::<super::SdtHeader>()) / if is_xsdt { 8 } else { 4 };
            let ptr = header.add(size_of::<super::SdtHeader>());
            RsdtTypeIter {
                ptr,
                is_xsdt,
                curr: 0,
                total,
            }
        }
    }
}

impl Rsdp {
    pub fn validate(&self) -> bool {
        let sum = unsafe {
            let bytes = core::slice::from_raw_parts(self as *const _ as *const u8, self.length());
            bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte))
        };

        sum == 0
    }

    pub fn signature(&self) -> &str {
        core::str::from_utf8(&self.signature).unwrap().trim()
    }

    pub fn oem_id(&self) -> &str {
        core::str::from_utf8(&self.oem_id).unwrap().trim()
    }

    /// If ACPI 1.0, return fixed size, else return length field
    pub fn length(&self) -> usize {
        match self.revision {
            0 => 20,
            _ => self.length as usize,
        }
    }

    pub fn as_type(&self) -> RsdtType {
        // This is fine.
        unsafe {
            match self.revision {
                0 => {
                    let addr = self.rsdt_addr as usize + amd64::paging::PHYS_VIRT_OFFSET;
                    RsdtType::Rsdt((addr as *const Rsdt).as_ref().unwrap())
                }
                _ => {
                    let addr = self.xsdt_addr as usize + amd64::paging::PHYS_VIRT_OFFSET;
                    RsdtType::Xsdt((addr as *const Xsdt).as_ref().unwrap())
                }
            }
        }
    }
}

impl core::fmt::Debug for Rsdp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("valid", &self.validate())
            .field("oem_id", &self.oem_id())
            .field("revision", &self.revision)
            .field("length", &self.length())
            .field("type", &self.as_type())
            .finish()
    }
}
