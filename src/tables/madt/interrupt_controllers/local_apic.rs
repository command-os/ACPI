use modular_bitfield::{bitfield, specifiers::*};

#[bitfield(bits = 32)]
#[repr(C, u32)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicFlags {
    #[skip(setters)]
    pub enabled: bool,
    #[skip(setters)]
    pub online_capable: bool,
    #[skip]
    reserved: B30,
}

#[repr(C, packed)]
pub struct LocalApic {
    header: super::IcHeader,
    acpi_uid: u8,
    apic_id: u8,
    flags: LocalApicFlags,
}

impl LocalApic {
    pub fn acpi_uid(&self) -> u8 {
        self.acpi_uid
    }

    pub fn apic_id(&self) -> u8 {
        self.apic_id
    }

    pub fn flags(&self) -> LocalApicFlags {
        self.flags
    }
}

impl core::ops::Deref for LocalApic {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for LocalApic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("header", &self.header)
            .field("acpi_uid", &self.acpi_uid())
            .field("apic_id", &self.apic_id())
            .field("flags", &self.flags())
            .finish()
    }
}
