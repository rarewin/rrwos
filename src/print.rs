use core::fmt;

use crate::console;
use crate::pl011::Uart;

// static mut UART: Uart = Uart::new();

pub fn _print(args: fmt::Arguments) {
    use console::interface::Write;

    let mut uart = Uart::new(); // FIXME: should be removed

    uart.write_fmt(args).unwrap();
}

// <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}
