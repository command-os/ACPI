use core::any::type_name;

#[repr(C, packed)]
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

impl core::fmt::Debug for LocalApicNmi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("header", &self.header)
            .field("acpi_proc_uid", &self.acpi_proc_uid())
            .field("flags", &self.flags())
            .field("local_apic_lint_num", &self.local_apic_lint_num())
            .finish()
    }
}
