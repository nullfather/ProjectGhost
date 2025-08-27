use core::sync::atomic::{AtomicUsize, Ordering};

const CAPACITY: usize = 4096;
static mut STORAGE: [u8; CAPACITY] = [0; CAPACITY];
static LEN: AtomicUsize = AtomicUsize::new(0);
const KEY: u8 = 0xAA;

pub fn create_vault() {}

pub fn store_model(data: &[u8]) {
    let len = data.len().min(CAPACITY);
    unsafe {
        for i in 0..len {
            STORAGE[i] = data[i] ^ KEY;
        }
    }
    LEN.store(len, Ordering::SeqCst);
}

pub fn load_model(buf: &mut [u8]) -> usize {
    let len = LEN.load(Ordering::SeqCst).min(buf.len());
    unsafe {
        for i in 0..len {
            buf[i] = STORAGE[i] ^ KEY;
        }
    }
    len
}
