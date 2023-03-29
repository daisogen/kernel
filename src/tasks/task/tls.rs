// Thread-local storage

use alloc::sync::Arc;
use core::sync::atomic::{AtomicUsize, Ordering};
use hashbrown::HashMap;
use spin::Mutex;

pub struct TLS {
    keys: Arc<Mutex<HashMap<usize, usize>>>,
    ctr: Arc<AtomicUsize>,
}

impl TLS {
    pub fn new() -> TLS {
        TLS {
            keys: Arc::new(Mutex::new(HashMap::new())),
            ctr: Arc::new(AtomicUsize::new(1)),
        }
    }

    pub fn create(&self) -> usize {
        let key = self.ctr.fetch_add(1, Ordering::SeqCst);
        self.keys.lock().insert(key, 0);
        key
    }

    pub fn set(&self, key: usize, val: usize) {
        self.keys.lock().insert(key, val);
    }

    pub fn get(&self, key: usize) -> usize {
        *self.keys.lock().get(&key).unwrap() // TODO: DO NOT UNWRAP!
    }

    pub fn destroy(&self, key: usize) {
        self.keys.lock().remove(&key);
        // TODO: implement destructors and call one here
    }
}

// ---

pub mod ffi {
    use crate::tasks::scheduler;
    pub extern "C" fn create() -> usize {
        scheduler::running_mut_task().tls.create()
    }
    pub extern "C" fn set(key: usize, val: usize) {
        scheduler::running_mut_task().tls.set(key, val);
    }
    pub extern "C" fn get(key: usize) -> usize {
        scheduler::running_mut_task().tls.get(key)
    }
    pub extern "C" fn destroy(key: usize) {
        scheduler::running_mut_task().tls.destroy(key);
    }
}
