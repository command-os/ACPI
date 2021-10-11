use core::any::type_name;

#[repr(C, packed)]
pub struct LocalApicAddrOverride {
    header: super::IcHeader,
    reserved: u16,
    local_apic_addr: u64,
}

impl LocalApicAddrOverride {
    pub fn local_apic_addr(&self) -> u64 {
        self.local_apic_addr
    }
}

impl core::ops::Deref for LocalApicAddrOverride {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for LocalApicAddrOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("header", &self.header)
            .field("local_apic_addr", &self.local_apic_addr())
            .finish()
    }
}
