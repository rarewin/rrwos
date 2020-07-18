#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("panic!");
    loop {}
}

#[cfg(target_arch = "arm")]
#[path = "cpu/armv7a/start.rs"]
pub mod start;

pub mod console;
pub mod pl011;
pub mod print;

#[no_mangle]
pub extern "C" fn _main() -> ! {
    let mut uart = pl011::Uart::new();

    uart.init(119200);

    // print::_print(format_args!("hogehoge"));

    println!("hogehoge");
    //
    //for c in b"Hello World".iter() {
    //    uart.write(*c as u32);
    //}

    panic!("test");

    loop {}
}
