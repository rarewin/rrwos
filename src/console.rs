use core::fmt;

use crate::console;
use crate::pl011::UartCore;
use crate::synchro::interface::Mutex;
use crate::synchro::NullLock;

static UART_INST: Uart = Uart::new();

pub struct Uart {
    core: NullLock<UartCore>,
}

impl Uart {
    const fn new() -> Self {
        Self {
            core: NullLock::new(UartCore::new()),
        }
    }
}

impl console::interface::Write for Uart {
    fn write_char(&self, c: char) {
        let mut core = &self.core;
        core.lock(|core| core.write_char(c));
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        let mut core = &self.core;
        core.lock(|core| fmt::Write::write_fmt(core, args))
    }

    fn flush(&self) {}
}

pub mod interface {
    pub use core::fmt;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }
}

pub fn console() -> &'static impl console::interface::Write {
    &UART_INST
}
