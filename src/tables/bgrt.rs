#[repr(C, packed)]
pub struct BGRT {
    header: super::SDTHeader,
    /// Must be 1
    version: u16,
    /// bits 2:1 orientation offset; 0b00 = no offset, 0b01 = 90, 0b10 = 180, 0b11 = 270
    status: u8,
    /// Must be 0
    image_type: u8,
    image_addr: u64,
    image_off: (u32, u32),
}

impl core::ops::Deref for BGRT {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}