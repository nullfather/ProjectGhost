use core::sync::atomic::{AtomicU8, Ordering};

static MODE: AtomicU8 = AtomicU8::new(0);

pub fn enable_isolation() { MODE.store(1, Ordering::SeqCst); }
pub fn enable_vpn() { MODE.store(2, Ordering::SeqCst); }
pub fn enable_tor() { MODE.store(3, Ordering::SeqCst); }

pub fn current_mode() -> u8 { MODE.load(Ordering::SeqCst) }
