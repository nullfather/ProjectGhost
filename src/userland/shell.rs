use super::sys_bindings;

fn print(s: &str) {
    unsafe { sys_bindings::write(1, s.as_ptr(), s.len() as u64); }
}

pub fn start() {
    print("ProjectGhost shell\n> ");
    let mut buf = [0u8; 64];
    loop {
        unsafe { sys_bindings::read(0, buf.as_mut_ptr(), buf.len() as u64); }
        match core::str::from_utf8(&buf).unwrap_or("").trim() {
            "ghost-mode" => {
                sys_bindings::enable_tor();
            }
            _ => {}
        }
    }
}
