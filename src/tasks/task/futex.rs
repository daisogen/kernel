use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;

pub struct Futexes {
    keys: Mutex<HashMap<usize, usize>>,
    f: Vec<Futex>,
}

impl Futexes {
    pub fn new() -> Futexes {
        Futexes {
            keys: Mutex::new(HashMap::new()),
            f: Vec::new(),
        }
    }

    fn var2key(&mut self, var: usize) -> usize {
        let mut guard = self.keys.lock();
        if !guard.contains_key(&var) {
            guard.insert(var, self.f.len());
            self.f.push(Futex::new(var));
        }
        let key = guard[&var];
        drop(guard);
        key
    }

    pub fn wait(&mut self, var: usize, val: usize) {
        let key = self.var2key(var);
        self.f[key].wait(val);
    }

    pub fn wake_one(&mut self, var: usize) {
        let key = self.var2key(var);
        self.f[key].wake_one();
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
    pub extern "C" fn wait(var: usize, val: usize) {
        scheduler::running_mut_task().futexes.wait(var, val)
    }
    pub extern "C" fn wake_one(var: usize) {
        scheduler::running_mut_task().futexes.wake_one(var)
    }
}
