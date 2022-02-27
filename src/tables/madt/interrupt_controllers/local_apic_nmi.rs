//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicNmi {
    header: super::IcHeader,
    pub acpi_proc_id: u8,
    pub flags: amd64::spec::mps::Inti,
    pub lint: u8,
}

impl core::ops::Deref for LocalApicNmi {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
