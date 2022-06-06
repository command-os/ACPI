//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::mem::size_of;

use modular_bitfield::prelude::*;

use self::ic::{ioapic::*, proc_lapic::*, *};

pub mod ic;

#[bitfield(bits = 32)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub struct MADTFlags {
    #[skip(setters)]
    pub pcat_compat: bool,
    #[skip]
    __: B31,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MADT {
    header: super::SDTHeader,
    local_ic_addr: u32,
    pub flags: MADTFlags,
}

pub struct MADTIter {
    ptr: *const u8,
    curr: usize,
    total: usize,
}

impl Iterator for MADTIter {
    type Item = InterruptController;

    fn next(&mut self) -> core::option::Option<<Self as core::iter::Iterator>::Item> {
        if self.curr == self.total {
            None
        } else {
            let next = unsafe { &*(self.ptr.add(self.curr) as *const ICHeader) };
            self.curr += next.length();
            unsafe {
                Some(match next.type_ {
                    0 => InterruptController::ProcessorLocalAPIC(
                        &*(next as *const _ as *const ProcessorLocalAPIC),
                    ),
                    1 => {
                        InterruptController::InputOutputAPIC(&*(next as *const _ as *const IOAPIC))
                    }
                    2 => InterruptController::InterruptSourceOverride(
                        &*(next as *const _ as *const InterruptSourceOverride),
                    ),
                    3 => InterruptController::NMISource(&*(next as *const _ as *const NMISource)),
                    4 => InterruptController::LocalApicNmi(
                        &*(next as *const _ as *const LocalAPICNMI),
                    ),
                    5 => InterruptController::LocalAPICAddrOverride(
                        &*(next as *const _ as *const LocalAPICAddrOverride),
                    ),
                    6 => InterruptController::InputOutputSAPIC(
                        &*(next as *const _ as *const ICHeader),
                    ),
                    7 => InterruptController::LocalSapic(&*(next as *const _ as *const ICHeader)),
                    8 => InterruptController::PlatformInterruptSrcs(
                        &*(next as *const _ as *const ICHeader),
                    ),
                    9 => InterruptController::ProcessorLocalx2APIC(
                        &*(next as *const _ as *const ICHeader),
                    ),
                    0xA => {
                        InterruptController::Localx2APICNmi(&*(next as *const _ as *const ICHeader))
                    }
                    0xB => InterruptController::GICCPU(&*(next as *const _ as *const ICHeader)),
                    0xC => InterruptController::GICDist(&*(next as *const _ as *const ICHeader)),
                    0xD => {
                        InterruptController::GICMSIFrame(&*(next as *const _ as *const ICHeader))
                    }
                    0xE => InterruptController::GICRedist(&*(next as *const _ as *const ICHeader)),
                    0xF => InterruptController::GICIts(&*(next as *const _ as *const ICHeader)),
                    0x10 => InterruptController::MPWakeup(&*(next as *const _ as *const ICHeader)),
                    0x11..=0x7F => {
                        InterruptController::Reserved(&*(next as *const _ as *const ICHeader))
                    }
                    0x80..=0xFF => {
                        InterruptController::OemReserved(&*(next as *const _ as *const ICHeader))
                    }
                })
            }
        }
    }
}

impl MADT {
    pub fn local_ic_addr(&self) -> u64 {
        self.local_ic_addr as u64
    }

    pub fn into_iter(&self) -> MADTIter {
        MADTIter {
            ptr: unsafe { (self as *const _ as *const u8).add(size_of::<Self>()) },
            curr: 0,
            total: self.length as usize - size_of::<Self>(),
        }
    }
}

impl core::ops::Deref for MADT {
    type Target = super::SDTHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
