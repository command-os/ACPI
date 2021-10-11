pub enum RSDTType {
    RSDT(&'static super::RSDT<'static, ()>),
    XSDT(&'static super::XSDT<'static, ()>),
}

#[repr(C, packed)]
pub struct RSDP {
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

impl RSDP {
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

    pub fn into_type(&self) -> RSDTType {
        // This is fine.
        unsafe {
            match self.revision {
                0 => RSDTType::RSDT(
                    &*((self.rsdt_addr as u64 + crate::PHYS_VIRT_OFFSET) as *const super::RSDT<()>),
                ),
                _ => RSDTType::XSDT(
                    &*((self.xsdt_addr + crate::PHYS_VIRT_OFFSET) as *const super::XSDT<()>),
                ),
            }
        }
    }
}

impl core::fmt::Debug for RSDP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RSDP")
            .field("valid", &self.validate())
            .field("oem_id", &self.oem_id())
            .field("revision", &self.revision())
            .field("length", &self.length())
            .finish()
    }
}
