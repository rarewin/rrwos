#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "arm")]
#[path = "cpu/armv7a/start.rs"]
pub mod start;

pub mod pl011;

#[no_mangle]
pub extern "C" fn _main() -> ! {
    let mut uart = pl011::Uart::new(119200);

    for c in b"Hello World".iter() {
        uart.write(*c as u32);
    }

    loop {}
}
