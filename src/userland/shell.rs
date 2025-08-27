use super::sys_bindings;

pub fn start() {
    let msg = b"ProjectGhost shell\n";
    unsafe {
        sys_bindings::write(1, msg.as_ptr(), msg.len() as u64);
    }
    loop {}
}
