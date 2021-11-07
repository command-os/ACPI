/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

pub use io_apic::*;
pub use iso::*;
pub use local_apic::*;
pub use local_apic_addr_override::*;
pub use local_apic_nmi::*;
pub use nmi_source::*;

mod io_apic;
mod iso;
mod local_apic;
mod local_apic_addr_override;
mod local_apic_nmi;
mod nmi_source;

#[derive(Debug)]
pub enum InterruptController {
    LocalApic(&'static LocalApic),
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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IcHeader {
    type_: u8,
    length: u8,
}

impl IcHeader {
    pub fn length(&self) -> usize {
        self.length.try_into().unwrap()
    }

    pub fn into_type(&self) -> InterruptController {
        unsafe {
            match self.type_ {
                0 => InterruptController::LocalApic(&*(self as *const _ as *const LocalApic)),
                1 => InterruptController::IoApic(&*(self as *const _ as *const IoApic)),
                2 => InterruptController::Iso(&*(self as *const _ as *const Iso)),
                3 => InterruptController::NmiSource(&*(self as *const _ as *const NmiSource)),
                4 => InterruptController::LocalApicNmi(&*(self as *const _ as *const LocalApicNmi)),
                5 => {
                    InterruptController::LocalApicAddrOverride(
                        &*(self as *const _ as *const LocalApicAddrOverride),
                    )
                }
                6 => InterruptController::IoSapic(&*(self as *const _ as *const IcHeader)),
                7 => InterruptController::LocalSapic(&*(self as *const _ as *const IcHeader)),
                8 => {
                    InterruptController::PlatformInterruptSrcs(
                        &*(self as *const _ as *const IcHeader),
                    )
                }
                9 => {
                    InterruptController::ProcessorLocalx2Apic(
                        &*(self as *const _ as *const IcHeader),
                    )
                }
                0xA => InterruptController::Localx2ApicNmi(&*(self as *const _ as *const IcHeader)),
                0xB => InterruptController::GicCpu(&*(self as *const _ as *const IcHeader)),
                0xC => InterruptController::GicDist(&*(self as *const _ as *const IcHeader)),
                0xD => InterruptController::GicMsiFrame(&*(self as *const _ as *const IcHeader)),
                0xE => InterruptController::GicRedist(&*(self as *const _ as *const IcHeader)),
                0xF => InterruptController::GicIts(&*(self as *const _ as *const IcHeader)),
                0x10 => InterruptController::MpWakeup(&*(self as *const _ as *const IcHeader)),
                0x11..=0x7F => {
                    InterruptController::Reserved(&*(self as *const _ as *const IcHeader))
                }
                0x80..=0xFF => {
                    InterruptController::OemReserved(&*(self as *const _ as *const IcHeader))
                }
            }
        }
    }
}
