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

pub struct UartCore {
    // reg: &'static mut Pl011Reg,
    written_chars: usize,
}

pub const PL011_BASE: u64 = 0x900_0000; // @todo should be obtained from dtc
pub const PL011_CLOCK: u32 = 0x16e_3600; // @todo should be obtained from dtc

impl fmt::Write for UartCore {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

impl UartCore {
    pub const fn new() -> Self {
        Self { written_chars: 0 }
    }

    pub fn write_char(&mut self, c: char) {
        unsafe {
            let reg = &mut *(PL011_BASE as *mut Pl011Reg);
            reg.dr.write(c as u32);
        }
        self.written_chars += 1;
    }
}

use crate::driver::interface::DeviceDriver;

impl DeviceDriver for UartCore {
    fn compatible(&self) -> &'static str {
        "pl011"
    }

    fn init(&mut self /* TODO: dtc */) -> Result<(), ()> {
        let baudrate = 115200; // TODO

        unsafe {
            let reg = &mut *(PL011_BASE as *mut Pl011Reg);
            reg.ibrd.write((PL011_CLOCK >> 4) / baudrate);
            reg.fbrd.write((PL011_CLOCK << 2) / baudrate);
            reg.ifls.write(0x12);
            reg.lcr_h.write(0x70);
            reg.cr.write(0x4301);
            reg.imsc.write(0x30);
        }

        Ok(())
    }
}
