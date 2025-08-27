//! Frame allocator and kernel heap setup.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::{Mutex, Once, MutexGuard};
use limine::{MemoryMapResponse, MemoryMapEntryType};
use x86_64::structures::paging::{PhysFrame, Size2MiB, FrameAllocator};
use x86_64::PhysAddr;

/// Simple bump allocator for the kernel heap.
struct BumpAllocator {
    next: usize,
    end: usize,
}

impl BumpAllocator {
    const fn empty() -> Self { Self { next: 0, end: 0 } }
    unsafe fn init(&mut self, start: usize, end: usize) {
        self.next = start;
        self.end = end;
    }
}

unsafe impl GlobalAlloc for Mutex<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut alloc = self.lock();
        let align = layout.align();
        let size = layout.size();
        let start = (alloc.next + align - 1) & !(align - 1);
        if start + size > alloc.end { return null_mut(); }
        alloc.next = start + size;
        start as *mut u8
    }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[global_allocator]
static HEAP: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::empty());

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! { panic!("allocation error"); }

/// Limine-provided memory map used as frame allocator.
struct LimineFrameAllocator {
    memmap: &'static MemoryMapResponse,
    next: usize,
}

impl LimineFrameAllocator {
    unsafe fn new(memmap: &'static MemoryMapResponse) -> Self {
        Self { memmap, next: 0 }
    }
}

unsafe impl FrameAllocator<Size2MiB> for LimineFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size2MiB>> {
        while self.next < self.memmap.entries().len() {
            let entry = &self.memmap.entries()[self.next];
            self.next += 1;
            if entry.typ == MemoryMapEntryType::Usable {
                return Some(PhysFrame::containing_address(PhysAddr::new(entry.base)));
            }
        }
        None
    }
}

static FRAME_ALLOCATOR: Once<Mutex<LimineFrameAllocator>> = Once::new();

/// Initialize the heap allocator and frame allocator using the provided memory map.
pub fn init(memmap: &'static MemoryMapResponse) {
    for entry in memmap.entries() {
        if entry.typ == MemoryMapEntryType::Usable && entry.length >= 1024 * 1024 {
            unsafe { HEAP.lock().init(entry.base as usize, (entry.base + entry.length) as usize); }
            break;
        }
    }
    FRAME_ALLOCATOR.call_once(|| Mutex::new(unsafe { LimineFrameAllocator::new(memmap) }));
}

/// Access the global frame allocator.
pub fn frame_allocator() -> MutexGuard<'static, LimineFrameAllocator> {
    FRAME_ALLOCATOR.get().expect("allocator not init").lock()
}
