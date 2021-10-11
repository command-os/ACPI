use core::any::type_name;

#[repr(C, packed)]
pub struct Iso {
    header: super::IcHeader,
    bus: u8,
    source: u8,
    gsi: u32,
    flags: amd64::spec::mps::Inti,
}

impl Iso {
    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn source(&self) -> u8 {
        self.source
    }

    pub fn gsi(&self) -> u32 {
        self.gsi
    }

    pub fn flags(&self) -> amd64::spec::mps::Inti {
        self.flags
    }
}

impl core::ops::Deref for Iso {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl core::fmt::Debug for Iso {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("bus", &self.bus())
            .field("source", &self.source())
            .field("gsi", &self.gsi())
            .field("flags", &self.flags())
            .finish()
    }
}
