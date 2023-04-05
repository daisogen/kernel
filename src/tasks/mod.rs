pub mod loader;
pub mod scheduler;
pub mod task;

use alloc::vec::Vec;
use spin::Mutex;
use task::Task;

pub type PID = usize;

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

    pids.pop().unwrap()
}

fn pid_to_base(pid: PID) -> u64 {
    crate::mem::HH + ((pid as u64) << 31)
}

fn base_to_pid(addr: u64) -> PID {
    ((addr - crate::mem::HH) >> 31) as PID
}

pub fn get_task(pid: PID) -> &'static Task {
    unsafe { &TASKS[pid] }
}

pub fn get_mut_task(pid: PID) -> &'static mut Task {
    unsafe { &mut TASKS[pid] }
}

// ---

// Get the return address for the function that calls this one. Then, convert
// that to a PID and get the task. This way, PD functions can know who
// called them, and thus which PID to lock.
#[inline(always)]
pub fn caller() -> PID {
    base_to_pid(unsafe { crate::utils::return_address(0) } as u64)
}

// Same but return a Task
#[inline(always)]
pub fn caller_mut_task() -> &'static mut Task {
    get_mut_task(caller())
}
