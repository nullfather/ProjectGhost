#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use limine::request::{BootloaderInfoRequest, MemoryMapRequest, KernelAddressRequest};
use limine::{BootloaderInfoResponse, MemoryMapResponse, KernelAddressResponse};
use x86_64::instructions::hlt;

mod paging;
mod idt;
mod syscalls;
mod memory;
mod scheduler;

#[path = "../drivers/disk.rs"]
pub mod disk;
#[path = "../drivers/net.rs"]
pub mod net;
#[path = "../drivers/display.rs"]
pub mod display;
#[path = "../userland/init.rs"]
pub mod init;
#[path = "../userland/shell.rs"]
pub mod shell;
#[path = "../userland/sys_bindings.rs"]
pub mod sys_bindings;

// Limine bootloader requests
static BOOT_INFO_REQ: BootloaderInfoRequest = BootloaderInfoRequest::new();
static MEMMAP_REQ: MemoryMapRequest = MemoryMapRequest::new();
static KERNEL_ADDR_REQ: KernelAddressRequest = KernelAddressRequest::new();

/// Kernel entry point. Control reaches here directly from the Limine bootloader.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Retrieve bootloader name and version for diagnostics.
    let boot_info: &BootloaderInfoResponse = BOOT_INFO_REQ
        .get_response()
        .get()
        .expect("Bootloader info missing");
    unsafe { display::print_boot_info(boot_info.name(), boot_info.version()); }

    // Obtain the memory map to configure the frame allocator and heap.
    let mem_map: &'static MemoryMapResponse = MEMMAP_REQ
        .get_response()
        .get()
        .expect("Memory map missing");
    memory::init(mem_map);

    // Acquire kernel physical load address for higher-half mapping.
    let kernel_addr: &KernelAddressResponse = KERNEL_ADDR_REQ
        .get_response()
        .get()
        .expect("Kernel address missing");
    paging::init(kernel_addr.phys_base); // Setup page tables.

    idt::init(); // Load IDT and enable interrupts.
    unsafe { x86_64::instructions::interrupts::enable(); }

    // Spawn initial task.
    scheduler::add_task(init::init_task);
    scheduler::run();
}

/// Panic handler prints diagnostic information then halts.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { display::print_panic(info); }
    loop { hlt(); }
}
