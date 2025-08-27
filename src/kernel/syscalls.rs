//! Syscall dispatcher for INT 0x80 interface.

use crate::disk;
use crate::net;

/// Dispatch system calls based on the value in RAX.
pub fn dispatch(num: u64) {
    match num {
        0x01 => disk::load_model(),
        0x02 => disk::unload_model(),
        0x03 => disk::run_inference(),
        0x04 => net::toggle_isolation(),
        _ => {},
    }
}
