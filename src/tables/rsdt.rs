use core::{marker::PhantomData, mem::size_of};

#[repr(C, packed)]
pub struct RSDT<'a, T: 'a> {
    header: super::SDTHeader,
    _phantom: PhantomData<&'a T>,
}

impl<'a> RSDT<'a, ()> {
    pub fn entries(&self) -> &'a [&'a super::SDTHeader] {
        let len = (self.length as usize - size_of::<super::SDTHeader>()) / 4;
        // This is very safe. Everything is fine here.
        unsafe {
            core::ptr::slice_from_raw_parts(
                (self as *const _ as *const u8).add(size_of::<super::SDTHeader>())
                    as *const &'a super::SDTHeader,
                len,
            )
            .as_ref()
            .unwrap()
        }
    }
}

impl<'a> core::ops::Deref for RSDT<'a, ()> {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl<'a> core::fmt::Debug for RSDT<'a, ()> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RSDT")
            .field("header", &self.header)
            .finish()
    }
}
