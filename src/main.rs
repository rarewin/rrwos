#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::write_volatile;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub const PL011_BASE: u64 = 0x900_0000;

pub const PL011_UARTIBRD: u64 = PL011_BASE + 0x024;
pub const PL011_UARTFBRD: u64 = PL011_BASE + 0x028;
pub const PL011_UARTIFLS: u64 = PL011_BASE + 0x034;
pub const PL011_UARTLCR_H: u64 = PL011_BASE + 0x02c;
pub const PL011_UARTCR: u64 = PL011_BASE + 0x030;
pub const PL011_UARTIMSC: u64 = PL011_BASE + 0x038;
pub const PL011_UARTDR: u64 = PL011_BASE + 0x000;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(PL011_UARTIBRD as *mut u32, 1);
        write_volatile(PL011_UARTFBRD as *mut u32, 0);
        write_volatile(PL011_UARTIFLS as *mut u32, 0x12);
        write_volatile(PL011_UARTLCR_H as *mut u32, 0x70);
        write_volatile(PL011_UARTCR as *mut u32, 0x4301);
        write_volatile(PL011_UARTIMSC as *mut u32, 0x30);
    }

    for c in b"Hello World".iter() {
        unsafe {
            write_volatile(PL011_UARTDR as *mut u32, *c as u32);
        }
    }

    loop {}
}
