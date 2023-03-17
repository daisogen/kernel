pub mod loader;
pub mod scheduler;
pub mod task;

use alloc::vec::Vec;
use spin::Mutex;
use task::Task;

type PID = usize;

static mut TASKS: Vec<Task> = Vec::new();
static PIDS: Mutex<Vec<PID>> = Mutex::new(Vec::new());
static MONOTONIC: Mutex<PID> = Mutex::new(1);

fn alloc_pid() -> PID {
    let mut pids = PIDS.lock();
    if pids.len() == 0 {
        let mut lock = MONOTONIC.lock();
        let ret = *lock;
        if ret == 1 {
            unsafe {
                TASKS.push(Task::new()); // Null task
            }
        }

        *lock += 1;
        unsafe {
            TASKS.push(Task::new()); // This (real) task
        }
        return ret;
    }

    return pids.pop().unwrap();
}

fn pid_to_base(pid: PID) -> u64 {
    crate::mem::HH + ((pid as u64) << 31)
}

pub fn get_task(pid: PID) -> &'static Task {
    return unsafe { &TASKS[pid] };
}

pub fn get_mut_task(pid: PID) -> &'static mut Task {
    return unsafe { &mut TASKS[pid] };
}
