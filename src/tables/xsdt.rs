/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use core::mem::size_of;

#[repr(C, packed)]
pub struct Xsdt {
    header: super::SdtHeader,
}

impl Xsdt {
    pub fn entries<'a>(&self) -> &'a [&'a super::SdtHeader] {
        let len = (self.length as usize - size_of::<Self>()) / 8;
        // This is very safe. Everything is fine here.
        unsafe {
            core::ptr::slice_from_raw_parts(
                (self as *const _ as *const u8).add(size_of::<Self>())
                    as *const &'a super::SdtHeader,
                len,
            )
            .as_ref()
            .unwrap()
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
            .field("entries", &self.entries())
            .finish()
    }
}
