pub mod loader;
pub mod task;

use alloc::vec::Vec;
use spin::Mutex;
use task::Task;

type PID = usize;

static TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static PIDS: Mutex<Vec<PID>> = Mutex::new(Vec::new());
static MONOTONIC: Mutex<PID> = Mutex::new(1);

fn alloc_pid() -> PID {
    let mut pids = PIDS.lock();
    if pids.len() == 0 {
        let mut lock = MONOTONIC.lock();
        let ret = *lock;
        if ret == 1 {
            TASKS.lock().push(Task::new()); // Null task
        }

        *lock += 1;
        TASKS.lock().push(Task::new()); // This (real) task
        return ret;
    }

    return pids.pop().unwrap();
}

fn pid_to_base(pid: PID) -> u64 {
    crate::mem::HH + ((pid as u64) << 31)
}
