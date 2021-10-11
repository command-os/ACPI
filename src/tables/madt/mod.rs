pub mod interrupt_controllers;

use core::mem::size_of;
use modular_bitfield::{bitfield, specifiers::*};

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

    pub fn entries<'a>(&self) -> &'a [&'a super::SdtHeader] {
        let len = (self.length as usize - size_of::<Self>()) / 8;
        // This is very safe. Everything is fine here.
        unsafe {
            core::ptr::slice_from_raw_parts(
                (self as *const _ as *const u8)
                    .add(size_of::<super::SdtHeader>() + amd64::paging::PHYS_VIRT_OFFSET as usize)
                    as *const &'a super::SdtHeader,
                len,
            )
            .as_ref()
            .unwrap()
        }
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
