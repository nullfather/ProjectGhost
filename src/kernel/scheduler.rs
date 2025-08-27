use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::structures::idt::InterruptStackFrame;

pub type Entry = extern "C" fn();

pub struct TaskControlBlock {
    pub entry: Entry,
    pub active: bool,
    pub pid: u32,
}

static mut TASKS: [TaskControlBlock; 4] = [
    TaskControlBlock { entry: idle, active: true, pid: 0 },
    TaskControlBlock { entry: idle, active: false, pid: 1 },
    TaskControlBlock { entry: idle, active: false, pid: 2 },
    TaskControlBlock { entry: idle, active: false, pid: 3 },
];

static CURRENT: AtomicUsize = AtomicUsize::new(0);

pub fn add_task(entry: Entry) -> Option<usize> {
    unsafe {
        for (i, t) in TASKS.iter_mut().enumerate().skip(1) {
            if !t.active {
                t.entry = entry;
                t.active = true;
                return Some(i);
            }
        }
    }
    None
}

pub extern "x86-interrupt" fn pit_handler(_stack: &mut InterruptStackFrame) {
    let next = (CURRENT.load(Ordering::SeqCst) + 1) % unsafe { TASKS.len() };
    CURRENT.store(next, Ordering::SeqCst);
}

pub fn current_task() -> Entry {
    unsafe { TASKS[CURRENT.load(Ordering::SeqCst)].entry }
}

extern "C" fn idle() {}
