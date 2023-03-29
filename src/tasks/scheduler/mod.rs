use crate::println;
use crate::tasks::PID;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use hashbrown::HashSet;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    // The round-robin
    static ref RR: Mutex<VecDeque<PID>> = Mutex::new(VecDeque::new());
    // Faster way to check if a PID is runnable
    static ref PRESENT: Mutex<HashSet<PID>> = Mutex::new(HashSet::new());
}

pub fn add(pid: PID) {
    let mut plock = PRESENT.lock();
    if plock.contains(&pid) {
        return;
    }

    let mut rrlock = RR.lock();
    rrlock.push_back(pid);
    plock.insert(pid);
}

pub fn schedule() -> ! {
    // Is there kernel saved state?
    crate::tasks::task::try_restore();

    let mut pid: PID = 0;
    {
        let mut plock = PRESENT.lock();
        let mut rrlock = RR.lock();
        let out = rrlock.pop_front();
        if let Some(out) = out {
            pid = out;
            plock.remove(&pid);
        }
    }; // Artificial block to scope locks

    if pid == 0 {
        todo!("Nothing to do");
    }

    println!("Boutta execute {}", pid);

    panic!("Hello");
}

// ---

lazy_static! {
    // Dispatcher sets this
    pub static ref RUNNING: Mutex<Vec<PID>> = {
        // RUNNING is accessed once IOAPIC has been initialized, and thus
        // ncores() returns the right value :)
        let mut v: Vec<PID> = Vec::new();
        v.resize(crate::utils::ncores(), 0);
        Mutex::new(v)
    };
}

pub fn running() -> PID {
    RUNNING.lock()[crate::utils::whoami()]
}

/*pub fn running_task() -> &'static super::Task {
    super::get_task(running())
}*/

pub fn running_mut_task() -> &'static mut super::Task {
    super::get_mut_task(running())
}
