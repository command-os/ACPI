mod local_apic;

pub use local_apic::*;

#[derive(Debug)]
pub enum InterruptController {
    LocalApic(&'static LocalApic),
    IoApic,
    Iso,
    NmiSource,
    LocalApicNmi,
    LocalApicAddrOverride,
    IoSapic,
    LocalSapic,
    PlatformInterruptSrcs,
    ProcessorLocalx2Apic,
    Localx2ApicNmi,
    GicCpu,
    GicDist,
    GicMsiFrame,
    GicRedist,
    GicIts,
    MpWakeup,
    Reserved,
    OemReserved,
}

#[repr(C, packed)]
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
                0 => InterruptController::LocalApic(
                    (self as *const _ as *const LocalApic).as_ref().unwrap(),
                ),
                1 => InterruptController::IoApic,
                2 => InterruptController::Iso,
                3 => InterruptController::NmiSource,
                4 => InterruptController::LocalApicNmi,
                5 => InterruptController::LocalApicAddrOverride,
                6 => InterruptController::IoSapic,
                7 => InterruptController::LocalSapic,
                8 => InterruptController::PlatformInterruptSrcs,
                9 => InterruptController::ProcessorLocalx2Apic,
                0xA => InterruptController::Localx2ApicNmi,
                0xB => InterruptController::GicCpu,
                0xC => InterruptController::GicDist,
                0xD => InterruptController::GicMsiFrame,
                0xE => InterruptController::GicRedist,
                0xF => InterruptController::GicIts,
                0x10 => InterruptController::MpWakeup,
                0x11..=0x7F => InterruptController::Reserved,
                0x80..=0xFF => InterruptController::OemReserved,
            }
        }
    }
}

impl core::fmt::Debug for IcHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("type", &self.type_)
            .field("length", &self.length())
            .finish_non_exhaustive()
    }
}
