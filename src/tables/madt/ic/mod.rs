//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use self::{ioapic::*, proc_lapic::*};

pub mod ioapic;
pub mod proc_lapic;

#[derive(Debug)]
pub enum InterruptController {
    ProcessorLocalApic(&'static ProcessorLocalApic),
    IoApic(&'static IoApic),
    Iso(&'static Iso),
    NmiSource(&'static NmiSource),
    LocalApicNmi(&'static LocalApicNmi),
    LocalApicAddrOverride(&'static LocalApicAddrOverride),
    IoSapic(&'static IcHeader),
    LocalSapic(&'static IcHeader),
    PlatformInterruptSrcs(&'static IcHeader),
    ProcessorLocalx2Apic(&'static IcHeader),
    Localx2ApicNmi(&'static IcHeader),
    GicCpu(&'static IcHeader),
    GicDist(&'static IcHeader),
    GicMsiFrame(&'static IcHeader),
    GicRedist(&'static IcHeader),
    GicIts(&'static IcHeader),
    MpWakeup(&'static IcHeader),
    Reserved(&'static IcHeader),
    OemReserved(&'static IcHeader),
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IcHeader {
    pub type_: u8,
    length: u8,
}

impl IcHeader {
    pub fn length(&self) -> usize {
        self.length.try_into().unwrap()
    }
}
