use core::fmt;

use volatile_register::{RO, RW, WO};

#[repr(C)]
struct Pl011Reg {
    dr: RW<u32>,
    rsr_ecr: RW<u32>,
    reserved_008_017: [RO<u32>; 4],
    fr: RO<u32>,
    reserved_01c_01f: [RO<u32>; 1],
    ilpr: RW<u32>,
    ibrd: RW<u32>,
    fbrd: RW<u32>,
    lcr_h: RW<u32>,
    cr: RW<u32>,
    ifls: RW<u32>,
    imsc: RW<u32>,
    ris: RO<u32>,
    mis: RO<u32>,
    icr: WO<u32>,
    dmacr: RW<u32>,
}

pub struct Uart {
    u: &'static mut Pl011Reg,
}

pub const PL011_BASE: u64 = 0x900_0000; // @todo should be obtained from dtc
pub const PL011_CLOCK: u32 = 0x16e_3600; // @todo should be obtained from dtc

impl Uart {
    pub fn new() -> Self {
        Self {
            u: unsafe { &mut *(PL011_BASE as *mut Pl011Reg) },
        }
    }

    pub fn init(&mut self, baudrate: u32) {
        unsafe {
            self.u.ibrd.write((PL011_CLOCK >> 4) / baudrate);
            self.u.fbrd.write((PL011_CLOCK << 2) / baudrate);
            self.u.ifls.write(0x12);
            self.u.lcr_h.write(0x70);
            self.u.cr.write(0x4301);
            self.u.imsc.write(0x30);
        }
    }

    pub fn write(&mut self, c: u32) {
        unsafe {
            self.u.dr.write(c);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write(c as u32);
        }

        Ok(())
    }
}
