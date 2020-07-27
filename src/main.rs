#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(trait_alias)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]

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

pub mod driver;

pub mod console;
pub mod print;
pub mod synchro;

pub mod pl011; // @todo temp

#[no_mangle]
pub extern "C" fn _main() -> ! {
    // print::_print(format_args!("hogehoge"));

    println!("hogehoge");

    //for c in b"Hello World".iter() {
    //    uart.write(*c as u32);
    //}

    panic!("test");
}
