//! Interrupt Descriptor Table configuration and handlers.

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use spin::Once;

use crate::{display, syscalls};

static IDT: Once<InterruptDescriptorTable> = Once::new();

/// Initialize and load the IDT.
pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(gpf_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        unsafe {
            idt[0x80]
                .set_handler_fn(syscall_entry)
                .set_privilege_level(x86_64::PrivilegeLevel::User);
        }
        idt
    });
    idt.load();
}

extern "x86-interrupt" fn page_fault_handler(stack: &mut InterruptStackFrame, error: PageFaultErrorCode) {
    unsafe { display::write_fmt(format_args!("PAGE FAULT {:?} at {:#?}\n", error, stack)); }
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn gpf_handler(stack: &mut InterruptStackFrame, error: u64) {
    unsafe { display::write_fmt(format_args!("GP FAULT {:#x} at {:#?}\n", error, stack)); }
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn double_fault_handler(stack: &mut InterruptStackFrame, _code: u64) {
    unsafe { display::write_fmt(format_args!("DOUBLE FAULT at {:#?}\n", stack)); }
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn syscall_entry(_stack: &mut InterruptStackFrame) {
    let num: u64;
    unsafe { core::arch::asm!("mov {}, rax", out(reg) num); }
    syscalls::dispatch(num);
}
