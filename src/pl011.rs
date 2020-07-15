use volatile_register::{RO, RW, WO};

#[repr(C)]
struct Pl011Reg {
    uartdr: RW<u32>,
    uartrsr: RW<u32>,
    reserved_008_017: [u32; 4],
    uartfw: RO<u32>,
    reserved_01c_01f: u32,
    uartilpr: RW<u32>,
    uartibrd: RW<u32>,
    uartfbrd: RW<u32>,
    uartlcr_h: RW<u32>,
    uartcr: RW<u32>,
    uartifls: RW<u32>,
    uartimsc: RW<u32>,
    uartris: RO<u32>,
    uartmis: RW<u32>,
    uarticr: WO<u32>,
    uartdmacr: RW<u32>,
}
