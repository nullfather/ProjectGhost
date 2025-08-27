//! Userland syscall bindings using INT 0x80.

/// Invoke a syscall with the given number.
pub fn syscall(num: u64) -> u64 {
    let ret: u64;
    unsafe { core::arch::asm!("int 0x80", in("rax") num, lateout("rax") ret); }
    ret
}

pub fn load_model() { let _ = syscall(0x01); }
pub fn unload_model() { let _ = syscall(0x02); }
pub fn run_inference() { let _ = syscall(0x03); }
pub fn toggle_net() { let _ = syscall(0x04); }
