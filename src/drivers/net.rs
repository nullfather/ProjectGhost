use core::sync::atomic::{AtomicU8, Ordering};

pub enum Mode {
    Normal = 0,
    Isolation = 1,
    Vpn = 2,
    Tor = 3,
}

static MODE: AtomicU8 = AtomicU8::new(0);

pub fn enable_isolation() { MODE.store(Mode::Isolation as u8, Ordering::SeqCst); }
pub fn enable_vpn() { MODE.store(Mode::Vpn as u8, Ordering::SeqCst); }
pub fn enable_tor() { MODE.store(Mode::Tor as u8, Ordering::SeqCst); }
pub fn disable() { MODE.store(Mode::Normal as u8, Ordering::SeqCst); }

pub fn current_mode() -> u8 { MODE.load(Ordering::SeqCst) }
