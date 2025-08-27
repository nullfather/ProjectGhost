//! First user process executed by the scheduler.

use crate::display;

pub fn init_task() {
    unsafe { display::write_fmt(format_args!("Init task running\n")); }
}
