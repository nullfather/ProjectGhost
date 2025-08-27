pub fn write(fd: u64, buf: *const u8, len: u64) -> u64 {
    syscall(0, fd, buf as u64, len, 0)
}

pub fn read(fd: u64, buf: *mut u8, len: u64) -> u64 {
    syscall(1, fd, buf as u64, len, 0)
}

pub fn open(path: *const u8, flags: u64) -> u64 {
    syscall(2, path as u64, flags, 0, 0)
}

pub fn close(fd: u64) -> u64 {
    syscall(3, fd, 0, 0, 0)
}

pub fn spawn(path: *const u8) -> u64 {
    syscall(4, path as u64, 0, 0, 0)
}

pub fn exit(code: u64) -> u64 {
    syscall(5, code, 0, 0, 0)
}

pub fn load_ai_model(name: *const u8, buf: *mut u8, len: u64) -> u64 {
    syscall(6, name as u64, buf as u64, len, 0)
}

#[inline(always)]
fn syscall(n: u64, a: u64, b: u64, c: u64, d: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") n,
            in("rdi") a,
            in("rsi") b,
            in("rdx") c,
            in("r10") d,
            lateout("rax") ret,
        );
    }
    ret
}
