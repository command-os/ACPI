/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use core::mem::size_of;

use modular_bitfield::prelude::*;

use crate::alloc::vec::Vec;

pub mod interrupt_controllers;

#[bitfield(bits = 32)]
#[repr(C, packed)]
pub struct MadtFlags {
    #[skip(setters)]
    pcat_compat: bool,
    #[skip]
    reserved: B31,
}

#[repr(C, packed)]
pub struct Madt {
    header: super::SdtHeader,
    local_ic_addr: u32,
    flags: MadtFlags,
}

impl Madt {
    pub fn local_ic_addr(&self) -> u64 {
        self.local_ic_addr as u64
    }

    pub fn has_pic(&self) -> bool {
        self.flags.pcat_compat()
    }

    pub fn into_ic_vec<'a>(&self) -> Vec<&'a interrupt_controllers::IcHeader> {
        let mut ret = Vec::new();
        let len = self.length as usize - size_of::<Self>();

        // Uhm. Sure
        unsafe {
            let mut ptr = (self as *const _ as *const u8).add(size_of::<Self>());
            let end = (self as *const _ as *const u8).add(size_of::<Self>() + len);

            while ptr != end {
                let ic = (ptr as *const interrupt_controllers::IcHeader)
                    .as_ref()
                    .unwrap();
                ret.push(ic);
                ptr = ptr.add(ic.length());
            }
        }

        ret
    }
}

impl core::ops::Deref for Madt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for Madt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("header", &self.header)
            .field("local_ic_addr", &self.local_ic_addr())
            .field("has_pic", &self.has_pic())
            .finish()
    }
}
