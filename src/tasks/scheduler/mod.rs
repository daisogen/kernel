use crate::tasks::PID;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::arch::global_asm;
use hashbrown::HashSet;
use lazy_static::lazy_static;
use spin::Mutex;

struct SchedulerState {
    // The round-robin
    rr: VecDeque<PID>,
    // PIDs that are in rr
    present: HashSet<PID>,
    // Index is CPU core; dispatcher sets this
    running: Vec<PID>,
}

lazy_static! {
    static ref STATE: Mutex<SchedulerState> = {
        Mutex::new(SchedulerState {
            rr: VecDeque::new(),
            present: HashSet::new(),
            running: {
                let mut v: Vec<PID> = Vec::new();
                v.resize(crate::utils::ncores(), 0);
                v
            },
        })
    };
}

pub fn add(pid: PID) {
    let mut state = STATE.lock();
    if state.present.contains(&pid) {
        // Already there!
        return;
    }

    state.rr.push_back(pid);
    state.present.insert(pid);
}

pub fn schedule() -> ! {
    let mut state = STATE.lock();
    if let Some(pid) = state.rr.pop_front() {
        // Got something!
        state.running[crate::utils::whoami()] = pid;
        state.present.remove(&pid);
        drop(state);
        crate::tasks::get_task(pid).dispatch();
        // That doesn't return
    } else {
        // Nothing to do, apparently
        state.running[crate::utils::whoami()] = 0;

        // Nothing to do? Is there kernel saved state?
        if unsafe { has_saved_kernel_state() } {
            // Going back!
            state.running[crate::utils::whoami()] = 0;
            drop(state);
            unsafe {
                restore_kernel_state();
            }
            // That does not return
        } else {
            // Definitely nothing to do 😴
            state.running[crate::utils::whoami()] = 0;
            drop(state);
            crate::utils::regs::sti();
            crate::utils::hlt();
            // That does not return
        }
    }
}

// ---

#[inline(never)]
pub fn schedule_saving() {
    // save_kernel_state() diverges. It can return two values:
    // false <==> kernel state has been saved correctly
    // true <==> kernel state has been restored!
    // That means that the function returns TWICE!!! Both states will be
    // returned in one single call! Kind of like fork() on POSIX returns
    // the PID of the child AND zero, but more confusing.
    if unsafe { save_kernel_state() } {
        return;
    } else {
        // Kernel state saved
        schedule();
    }
}

extern "C" {
    fn save_kernel_state() -> bool;
    pub fn has_saved_kernel_state() -> bool;
    pub fn restore_kernel_state() -> !;
}

global_asm!(include_str!("kstatesave.s"));

// ---

pub fn core_running() -> PID {
    STATE.lock().running[crate::utils::whoami()]
}

/*pub fn real_running_task() -> &'static super::Task {
    super::get_task(running())
}*/

/*pub fn real_running_mut_task() -> &'static mut super::Task {
    super::get_mut_task(running())
}*/
