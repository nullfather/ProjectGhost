#![no_std]
#![no_main]

use core::arch::asm;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr3, Cr4, Cr4Flags};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::structures::paging::PhysFrame;
use x86_64::PhysAddr;

mod syscalls;
mod scheduler;
mod mem;
#[path = "../drivers/disk.rs"]
pub mod disk;
#[path = "../drivers/net.rs"]
pub mod net;
#[path = "../userland/mod.rs"]
mod userland;

#[repr(align(4096))]
struct PageTable([u64; 512]);

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const HUGE: u64 = 1 << 7;

static mut PML4: PageTable = PageTable([0; 512]);
static mut PDP: PageTable = PageTable([0; 512]);
static mut PD: PageTable = PageTable([0; 512]);

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[repr(C)]
struct LimineBootloaderInfoResponse {
    revision: u64,
    name: *const u8,
    version: *const u8,
}

#[repr(C)]
struct LimineBootloaderInfoRequest {
    id: [u64; 2],
    revision: u64,
    response: *const LimineBootloaderInfoResponse,
}

#[link_section = ".limine_requests"]
#[used]
static BOOTLOADER_INFO_REQUEST: LimineBootloaderInfoRequest = LimineBootloaderInfoRequest {
    id: [0xf55038d8e2a1202f, 0x279426fcf5f59740],
    revision: 0,
    response: core::ptr::null(),
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        mem::init();
        paging_init();
        idt_init();
        pit_init();
    }
    interrupts::enable();

    userland::init::init();

    loop {
        unsafe { asm!("hlt"); }
    }
}

unsafe fn paging_init() {
    PML4.0[0] = (&PDP as *const _ as u64) | PRESENT | WRITABLE;
    PDP.0[0] = (&PD as *const _ as u64) | PRESENT | WRITABLE;
    for i in 0..512 {
        PD.0[i] = (i as u64 * 0x200000) | PRESENT | WRITABLE | HUGE;
    }
    let pml4_phys = PhysAddr::new(&PML4 as *const _ as u64);
    let (_, flags) = Cr3::read();
    Cr3::write(PhysFrame::containing_address(pml4_phys), flags);
    Cr4::update(|f| *f |= Cr4Flags::PAGE_SIZE_EXTENSIONS);
    Cr0::update(|f| *f |= Cr0Flags::PAGING);
}

unsafe fn idt_init() {
    IDT.divide_error.set_handler_fn(exception);
    IDT.general_protection_fault.set_handler_fn(exception);
    IDT.page_fault.set_handler_fn(exception);
    IDT[32].set_handler_fn(scheduler::pit_handler);
    IDT[0x80].set_handler_fn(syscall_handler);
    IDT.load();
}

unsafe fn pit_init() {
    let mut command = Port::new(0x43u16);
    let mut data = Port::new(0x40u16);
    command.write(0x36u8);
    data.write(0u8);
    data.write(0u8);
}

extern "x86-interrupt" fn exception(_stack: &mut InterruptStackFrame) {}

extern "x86-interrupt" fn syscall_handler(_stack: &mut InterruptStackFrame) {
    let num: u64; let a: u64; let b: u64; let c: u64; let d: u64;
    unsafe {
        asm!("mov {}, rax", out(reg) num);
        asm!("mov {}, rdi", out(reg) a);
        asm!("mov {}, rsi", out(reg) b);
        asm!("mov {}, rdx", out(reg) c);
        asm!("mov {}, r10", out(reg) d);
        let ret = syscalls::dispatch(num, a, b, c, d);
        asm!("mov rax, {}", in(reg) ret);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt"); }
    }
}
