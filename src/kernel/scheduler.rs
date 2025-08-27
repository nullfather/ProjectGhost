//! Minimal round-robin task scheduler.

use spin::Mutex;

type Task = fn();

static TASKS: Mutex<[Option<Task>; 16]> = Mutex::new([None; 16]);
static CURRENT: Mutex<usize> = Mutex::new(0);

/// Add a task to the scheduler.
pub fn add_task(task: Task) {
    let mut tasks = TASKS.lock();
    for slot in tasks.iter_mut() {
        if slot.is_none() { *slot = Some(task); break; }
    }
}

/// Run tasks in a simple round-robin fashion.
pub fn run() -> ! {
    loop {
        let task_opt = {
            let tasks = TASKS.lock();
            let mut cur = CURRENT.lock();
            let task = tasks[*cur];
            *cur = (*cur + 1) % tasks.len();
            task
        };
        if let Some(task) = task_opt { task(); }
        x86_64::instructions::hlt();
    }
}
