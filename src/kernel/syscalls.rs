use core::ptr;

pub type SysResult = u64;

pub const SYS_WRITE: u64 = 0;
pub const SYS_READ: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_SPAWN: u64 = 4;
pub const SYS_EXIT: u64 = 5;
pub const SYS_LOAD_AI_MODEL: u64 = 6;

pub fn dispatch(num: u64, a: u64, b: u64, c: u64, d: u64) -> SysResult {
    match num {
        SYS_WRITE => sys_write(a, b as *const u8, c as usize),
        SYS_READ => sys_read(a, b as *mut u8, c as usize),
        SYS_OPEN => sys_open(a as *const u8, b as u64),
        SYS_CLOSE => sys_close(a),
        SYS_SPAWN => sys_spawn(a as *const u8),
        SYS_EXIT => sys_exit(a),
        SYS_LOAD_AI_MODEL => sys_load_ai_model(a as *const u8, b as *mut u8, c as usize),
        _ => 0,
    }
}

fn sys_write(_fd: u64, _buf: *const u8, _len: usize) -> SysResult {
    // stub write
    0
}

fn sys_read(_fd: u64, _buf: *mut u8, _len: usize) -> SysResult {
    0
}

fn sys_open(_path: *const u8, _flags: u64) -> SysResult {
    0
}

fn sys_close(_fd: u64) -> SysResult {
    0
}

fn sys_spawn(_path: *const u8) -> SysResult {
    0
}

fn sys_exit(_code: u64) -> SysResult {
    loop {}
}

fn sys_load_ai_model(_name: *const u8, _buf: *mut u8, _len: usize) -> SysResult {
    // stub: just zero out buffer
    for i in 0.._len {
        unsafe { ptr::write(_buf.add(i), 0); }
    }
    0
}
