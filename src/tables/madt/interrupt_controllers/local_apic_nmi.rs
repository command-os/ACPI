/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LocalApicNmi {
    header: super::IcHeader,
    acpi_proc_uid: u8,
    flags: amd64::spec::mps::Inti,
    local_apic_lint_num: u8,
}

impl LocalApicNmi {
    pub fn acpi_proc_uid(&self) -> u8 {
        self.acpi_proc_uid
    }

    pub fn flags(&self) -> amd64::spec::mps::Inti {
        self.flags
    }

    pub fn local_apic_lint_num(&self) -> u8 {
        self.local_apic_lint_num
    }
}

impl core::ops::Deref for LocalApicNmi {
    type Target = super::IcHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
