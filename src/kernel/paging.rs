//! Paging and MMU setup. Establishes identity mapping for the lower 1GiB
//! and higher-half mapping for the kernel.

use x86_64::structures::paging::{Page, PageTable, PhysFrame, Mapper, Size2MiB, PageTableFlags, FrameAllocator, OffsetPageTable};
use x86_64::registers::control::{Cr3, Cr4, Cr4Flags};
use x86_64::{VirtAddr, PhysAddr};

use crate::memory;

pub const PRESENT: u64 = 1 << 0;
pub const WRITABLE: u64 = 1 << 1;
pub const HUGE: u64 = 1 << 7;
pub const KERNEL_BASE: u64 = 0xFFFFFFFF80000000;

/// Create a new OffsetPageTable from the active level 4 table.
unsafe fn init_offset_table() -> OffsetPageTable<'static> {
    let (frame, _) = Cr3::read();
    let phys = frame.start_address();
    let virt = VirtAddr::new(phys.as_u64());
    let table: &mut PageTable = &mut *virt.as_mut_ptr();
    OffsetPageTable::new(table, VirtAddr::new(KERNEL_BASE))
}

/// Initialize paging by creating identity and higher-half mappings.
pub fn init(kernel_phys_base: u64) {
    unsafe { Cr4::update(|f| f.insert(Cr4Flags::PHYSICAL_ADDRESS_EXTENSION)); }
    let mut mapper = unsafe { init_offset_table() };
    let mut allocator = memory::frame_allocator();

    // Identity map first 1GiB using 2MiB pages.
    let start = PhysFrame::containing_address(PhysAddr::new(0));
    let end = PhysFrame::containing_address(PhysAddr::new(1024 * 1024 * 1024));
    for frame in PhysFrame::<Size2MiB>::range(start, end) {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE;
        unsafe { mapper.identity_map(frame, flags, &mut *allocator).unwrap().flush(); }
    }

    // Higher-half mapping for the kernel image.
    let kernel_start = PhysFrame::containing_address(PhysAddr::new(kernel_phys_base));
    let virt_start = VirtAddr::new(KERNEL_BASE);
    for i in 0..16u64 { // map first 32MiB for kernel
        let frame = kernel_start + i;
        let page = Page::containing_address(virt_start + i * Size2MiB::SIZE);
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE;
        unsafe { mapper.map_to(page, frame, flags, &mut *allocator).unwrap().flush(); }
    }
}
