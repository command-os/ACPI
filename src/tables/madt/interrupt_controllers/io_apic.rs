#[repr(C, packed)]
pub struct IoApic {
    header: super::IcHeader,
    io_apic_id: u8,
    reserved: u8,
    io_apic_address: u32,
    gsi_base: u32,
}

impl IoApic {
    pub fn io_apic_id(&self) -> u8 {
        self.io_apic_id
    }

    pub fn io_apic_address(&self) -> u32 {
        self.io_apic_address
    }

    pub fn gsi_base(&self) -> u32 {
        self.gsi_base
    }
}

impl core::ops::Deref for IoApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for IoApic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("header", &self.header)
            .field("io_apic_id", &self.io_apic_id())
            .field("io_apic_address", &self.io_apic_address())
            .field("gsi_base", &self.gsi_base())
            .finish()
    }
}
