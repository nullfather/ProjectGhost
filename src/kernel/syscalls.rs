use core::ptr;
use crate::{disk, scheduler};

pub type SysResult = u64;

pub const SYS_WRITE: u64 = 0;
pub const SYS_READ: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_SPAWN: u64 = 4;
pub const SYS_EXIT: u64 = 5;
pub const SYS_LOAD_AI_MODEL: u64 = 6;
pub const SYS_ENABLE_ISOLATION: u64 = 7;
pub const SYS_ENABLE_VPN: u64 = 8;
pub const SYS_ENABLE_TOR: u64 = 9;
pub const SYS_GET_NET_MODE: u64 = 10;

pub fn dispatch(num: u64, a: u64, b: u64, c: u64, d: u64) -> SysResult {
    match num {
        SYS_WRITE => sys_write(a, b as *const u8, c as usize),
        SYS_READ => sys_read(a, b as *mut u8, c as usize),
        SYS_OPEN => sys_open(a as *const u8, b as u64),
        SYS_CLOSE => sys_close(a),
        SYS_SPAWN => sys_spawn(a as *const u8),
        SYS_EXIT => sys_exit(a),
        SYS_LOAD_AI_MODEL => sys_load_ai_model(a as *const u8, b as *mut u8, c as usize),
        SYS_ENABLE_ISOLATION => { crate::net::enable_isolation(); 0 }
        SYS_ENABLE_VPN => { crate::net::enable_vpn(); 0 }
        SYS_ENABLE_TOR => { crate::net::enable_tor(); 0 }
        SYS_GET_NET_MODE => crate::net::current_mode() as u64,
        _ => 0,
    }
}

fn sys_write(_fd: u64, _buf: *const u8, _len: usize) -> SysResult {
    let slice = unsafe { core::slice::from_raw_parts(_buf, _len) };
    let vga = 0xb8000 as *mut u8;
    for (i, b) in slice.iter().enumerate() {
        unsafe {
            ptr::write(vga.add(i * 2), *b);
            ptr::write(vga.add(i * 2 + 1), 0x07);
        }
    }
    _len as u64
}

fn sys_read(_fd: u64, _buf: *mut u8, _len: usize) -> SysResult {
    for i in 0.._len {
        unsafe { ptr::write(_buf.add(i), 0); }
    }
    0
}

fn sys_open(_path: *const u8, _flags: u64) -> SysResult {
    if _path.is_null() { return 0; }
    1
}

fn sys_close(_fd: u64) -> SysResult {
    let _ = _fd;
    0
}

fn sys_spawn(_path: *const u8) -> SysResult {
    let entry = unsafe { core::mem::transmute::<*const u8, scheduler::Entry>(_path) };
    if scheduler::add_task(entry).is_some() { 0 } else { 1 }
}

fn sys_exit(_code: u64) -> SysResult {
    let _ = _code;
    loop {}
}

fn sys_load_ai_model(_name: *const u8, _buf: *mut u8, _len: usize) -> SysResult {
    let name = if _name.is_null() { &[][..] } else { unsafe {
        let mut len = 0;
        while ptr::read(_name.add(len)) != 0 { len += 1; }
        core::slice::from_raw_parts(_name, len)
    } };
    let _ = name;
    let slice = unsafe { core::slice::from_raw_parts_mut(_buf, _len) };
    disk::load_model(slice) as u64
}
