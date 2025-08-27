use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 64 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn init() {}

pub fn kmalloc(size: usize) -> *mut u8 {
    let mut current = NEXT.load(Ordering::SeqCst);
    loop {
        if current + size > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        match NEXT.compare_exchange(current, current + size, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => unsafe { return HEAP.as_mut_ptr().add(current); },
            Err(v) => current = v,
        }
    }
}

pub fn kfree(_ptr: *mut u8) {}
