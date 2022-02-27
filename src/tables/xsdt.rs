//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use alloc::vec::Vec;
use core::mem::size_of;

#[repr(C, packed)]
pub struct Xsdt {
    header: super::SdtHeader,
}

#[derive(Debug)]
pub struct XsdtIterator {
    ptr: *const u64,
    curr: usize,
    total: usize,
}

impl Iterator for XsdtIterator {
    type Item = &'static super::SdtHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.total {
            None
        } else {
            unsafe {
                let next = (self.ptr.add(self.curr).read() as usize
                    + amd64::paging::PHYS_VIRT_OFFSET)
                    as *const super::SdtHeader;
                self.curr += 1;
                next.as_ref()
            }
        }
    }
}

impl Xsdt {
    pub fn iter(&self) -> XsdtIterator {
        let total = (self.length() - size_of::<Self>()) / 8;
        let ptr = unsafe { (self as *const _ as *const u8).add(size_of::<Self>()) as *const u64 };
        XsdtIterator {
            curr: 0,
            total,
            ptr,
        }
    }
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
            .field("entries", &self.iter().collect::<Vec<_>>())
            .finish()
    }
}
