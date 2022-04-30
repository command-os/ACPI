//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::mem::size_of;

use modular_bitfield::prelude::*;

use self::ic::{ioapic::*, proc_lapic::*, *};

pub mod ic;

#[bitfield(bits = 32)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub struct MadtFlags {
    #[skip(setters)]
    pub pcat_compat: bool,
    #[skip]
    __: B31,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Madt {
    header: super::SdtHeader,
    local_ic_addr: u32,
    pub flags: MadtFlags,
}

pub struct MadtIterator {
    ptr: *const u8,
    curr: usize,
    total: usize,
}

impl Iterator for MadtIterator {
    type Item = InterruptController;

    fn next(&mut self) -> core::option::Option<<Self as core::iter::Iterator>::Item> {
        if self.curr == self.total {
            None
        } else {
            let next = unsafe { &*(self.ptr.add(self.curr) as *const IcHeader) };
            self.curr += next.length();
            unsafe {
                Some(match next.type_ {
                    0 => InterruptController::ProcessorLocalApic(&*(next as *const _ as *const ProcessorLocalApic)),
                    1 => InterruptController::IoApic(&*(next as *const _ as *const IoApic)),
                    2 => InterruptController::Iso(&*(next as *const _ as *const Iso)),
                    3 => InterruptController::NmiSource(&*(next as *const _ as *const NmiSource)),
                    4 => {
                        InterruptController::LocalApicNmi(
                            &*(next as *const _ as *const LocalApicNmi),
                        )
                    }
                    5 => {
                        InterruptController::LocalApicAddrOverride(
                            &*(next as *const _ as *const LocalApicAddrOverride),
                        )
                    }
                    6 => InterruptController::IoSapic(&*(next as *const _ as *const IcHeader)),
                    7 => InterruptController::LocalSapic(&*(next as *const _ as *const IcHeader)),
                    8 => {
                        InterruptController::PlatformInterruptSrcs(
                            &*(next as *const _ as *const IcHeader),
                        )
                    }
                    9 => {
                        InterruptController::ProcessorLocalx2Apic(
                            &*(next as *const _ as *const IcHeader),
                        )
                    }
                    0xA => {
                        InterruptController::Localx2ApicNmi(&*(next as *const _ as *const IcHeader))
                    }
                    0xB => InterruptController::GicCpu(&*(next as *const _ as *const IcHeader)),
                    0xC => InterruptController::GicDist(&*(next as *const _ as *const IcHeader)),
                    0xD => {
                        InterruptController::GicMsiFrame(&*(next as *const _ as *const IcHeader))
                    }
                    0xE => InterruptController::GicRedist(&*(next as *const _ as *const IcHeader)),
                    0xF => InterruptController::GicIts(&*(next as *const _ as *const IcHeader)),
                    0x10 => InterruptController::MpWakeup(&*(next as *const _ as *const IcHeader)),
                    0x11..=0x7F => {
                        InterruptController::Reserved(&*(next as *const _ as *const IcHeader))
                    }
                    0x80..=0xFF => {
                        InterruptController::OemReserved(&*(next as *const _ as *const IcHeader))
                    }
                })
            }
        }
    }
}

impl Madt {
    pub fn local_ic_addr(&self) -> u64 {
        self.local_ic_addr as u64
    }

    pub fn into_iter(&self) -> MadtIterator {
        MadtIterator {
            ptr: unsafe { (self as *const _ as *const u8).add(size_of::<Self>()) },
            curr: 0,
            total: self.length as usize - size_of::<Self>(),
        }
    }
}

impl core::ops::Deref for Madt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
