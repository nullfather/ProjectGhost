//! Basic VGA text mode output for diagnostics.

use core::fmt::{self, Write};
use spin::Mutex;

const VGA_BUFFER: usize = 0xb8000;

struct Writer {
    column: usize,
}

impl Writer {
    const fn new() -> Self { Self { column: 0 } }
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            core::ptr::write_volatile((VGA_BUFFER + self.column * 2) as *mut u8, byte);
            core::ptr::write_volatile((VGA_BUFFER + self.column * 2 + 1) as *mut u8, 0x07);
        }
        self.column = (self.column + 1) % (80 * 25);
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() { self.write_byte(b); }
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

/// Print bootloader information.
pub unsafe fn print_boot_info(name: &str, version: &str) {
    let mut w = WRITER.lock();
    let _ = writeln!(w, "Bootloader: {} {}", name, version);
}

/// Print panic information.
pub unsafe fn print_panic(info: &core::panic::PanicInfo) {
    let mut w = WRITER.lock();
    let _ = writeln!(w, "PANIC: {}", info);
}

/// Generic formatted output helper used by interrupt handlers.
pub unsafe fn write_fmt(args: fmt::Arguments) {
    let mut w = WRITER.lock();
    let _ = w.write_fmt(args);
}
