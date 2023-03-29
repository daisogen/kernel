use crate::tasks::{self, scheduler, PID};
use alloc::boxed::Box;
use alloc::collections::VecDeque;

struct Futex {
    pub access: spin::Mutex<()>,
    pub var: &'static usize,
    pub parking: VecDeque<PID>,
}

pub extern "C" fn new(var: u64) -> u64 {
    let ret: Box<Futex> = Box::new(Futex {
        access: spin::Mutex::new(()),
        var: unsafe { &*(var as *const usize) },
        parking: VecDeque::new(),
    });

    let ret = Box::leak(ret) as *mut Futex as u64;
    tasks::get_mut_task(scheduler::get_running())
        .mutexes
        .insert(ret);
    ret
}

pub extern "C" fn wait(ptr: u64, val: usize) {
    let futex: &mut Futex = unsafe { &mut *(ptr as *mut Futex) };
    let _guard = futex.access.lock();

    if *futex.var == val {
        // zzZZzz
        futex.parking.push_back(scheduler::get_running());
        drop(_guard);
        scheduler::schedule();
        // That does not return
    } else {
        // No need to sleep!
    }
}

pub extern "C" fn wake_one(ptr: u64) {
    let futex: &mut Futex = unsafe { &mut *(ptr as *mut Futex) };
    let _guard = futex.access.lock();

    if let Some(pid) = futex.parking.pop_front() {
        // wake
        scheduler::add(pid);
    }
}
