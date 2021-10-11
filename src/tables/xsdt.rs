use core::mem::size_of;

#[repr(C, packed)]
pub struct XSDT {
    header: super::SDTHeader,
}

impl XSDT {
    pub fn entries<'a>(&self) -> &'a [&'a super::SDTHeader] {
        let len = (self.length as usize - size_of::<super::SDTHeader>()) / 8;
        // This is very safe. Everything is fine here.
        unsafe {
            core::ptr::slice_from_raw_parts(
                (self as *const _ as *const u8)
                    .add(size_of::<super::SDTHeader>() + amd64::paging::PHYS_VIRT_OFFSET as usize)
                    as *const &'a super::SDTHeader,
                len,
            )
            .as_ref()
            .unwrap()
        }
    }
}

impl core::ops::Deref for XSDT {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for XSDT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RSDT")
            .field("header", &self.header)
            .finish()
    }
}
