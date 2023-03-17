use crate::println;
use crate::tasks::PID;
use alloc::collections::vec_deque::VecDeque;
use hashbrown::HashSet;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref RR: Mutex<VecDeque<PID>> = Mutex::new(VecDeque::new());
    static ref PRESENT: Mutex<HashSet<PID>> = Mutex::new(HashSet::new());
}

/*pub fn add(pid: PID) {
    let mut plock = PRESENT.lock();
    if plock.contains(&pid) {
        return;
    }

    let mut rrlock = RR.lock();
    rrlock.push_back(pid);
    plock.insert(pid);
}*/

pub fn schedule() -> ! {
    // Is there kernel saved state?
    crate::tasks::task::try_restore();

    let mut pid: PID = 0;
    {
        let mut plock = PRESENT.lock();
        let mut rrlock = RR.lock();
        let out = rrlock.pop_front();
        if out.is_some() {
            pid = out.unwrap();
            plock.remove(&pid);
        }
    };

    if pid == 0 {
        todo!("Nothing to do");
    }

    println!("Boutta execute {}", pid);

    panic!("Hello");
}
