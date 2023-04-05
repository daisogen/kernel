// Thread-local storage

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

pub struct TLS {
    keys: Arc<Mutex<Vec<usize>>>,
}

impl TLS {
    pub fn new() -> TLS {
        TLS {
            keys: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn create(&self) -> usize {
        let mut guard = self.keys.lock();
        let key = guard.len();
        guard.push(0); // Default value
        key
    }

    pub fn set(&self, key: usize, val: usize) {
        self.keys.lock()[key] = val;
    }

    pub fn get(&self, key: usize) -> usize {
        self.keys.lock()[key]
    }

    pub fn destroy(&self, _key: usize) {
        // TODO: implement destructors and call one here
    }
}

// ---

pub mod ffi {
    use crate::tasks;

    pub extern "C" fn create() -> usize {
        tasks::caller_mut_task().tls.create()
    }
    pub extern "C" fn set(key: usize, val: usize) {
        tasks::caller_mut_task().tls.set(key, val);
    }
    pub extern "C" fn get(key: usize) -> usize {
        tasks::caller_mut_task().tls.get(key)
    }
    pub extern "C" fn destroy(key: usize) {
        tasks::caller_mut_task().tls.destroy(key);
    }
}
