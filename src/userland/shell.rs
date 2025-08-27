//! Minimal CLI shell placeholder.

use crate::display;

pub fn shell_task() {
    unsafe { display::write_fmt(format_args!("Shell ready\n")); }
}
