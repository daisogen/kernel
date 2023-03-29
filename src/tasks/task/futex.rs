use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

// Only handling one futex at a time????
pub struct Futexes {
    keys: Arc<Mutex<Vec<Box<Futex>>>>,
}

impl Futexes {
    pub fn new() -> Futexes {
        Futexes {
            keys: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn create(&self, var: usize) -> usize {
        let mut guard = self.keys.lock();
        let ret = guard.len();
        guard.push(Box::new(Futex::new(var)));
        ret
    }

    pub fn wait(&self, key: usize, val: usize) {
        self.keys.lock()[key].wait(val);
    }

    pub fn wake_one(&self, key: usize) {
        self.keys.lock()[key].wake_one();
    }
}

// ---

use crate::tasks::scheduler;
use crate::tasks::PID;
use alloc::collections::VecDeque;

pub struct Futex {
    access: spin::Mutex<()>,
    var: &'static usize,
    parking: VecDeque<PID>,
}

impl Futex {
    pub fn new(var: usize) -> Futex {
        Futex {
            access: spin::Mutex::new(()),
            var: unsafe { &*(var as *const usize) },
            parking: VecDeque::new(),
        }
    }

    pub fn wait(&mut self, val: usize) {
        let _guard = self.access.lock();

        if *self.var == val {
            // zzZZzz
            self.parking.push_back(scheduler::running());
            drop(_guard);
            scheduler::schedule();
            // That does not return
        } else {
            // No need to sleep!
        }
    }

    pub fn wake_one(&mut self) {
        let _guard = self.access.lock();

        if let Some(pid) = self.parking.pop_front() {
            // wake
            scheduler::add(pid);
        }
    }
}

// ---

pub mod ffi {
    use crate::tasks::scheduler;
    pub extern "C" fn new(var: usize) -> usize {
        scheduler::running_mut_task().futexes.create(var)
    }
    pub extern "C" fn wait(key: usize, val: usize) {
        scheduler::running_mut_task().futexes.wait(key, val)
    }
    pub extern "C" fn wake_one(key: usize) {
        scheduler::running_mut_task().futexes.wake_one(key)
    }
}
