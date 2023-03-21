// This file describes the Pointer Directory

use crate::mem::paging::{Paging, PAGING};
use crate::mem::pmm;
use crate::utils::strptr2str;
use alloc::string::String;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::Mutex;

// This is the whole point
lazy_static! {
    static ref PD: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

// The first page of the higher half guides the pointer directory paradigm
const PD_START: u64 = crate::mem::HH;

pub fn init() {
    // Get a physical page for the start of the pointer directory
    let phys = pmm::calloc(1);
    let phys = phys.expect("no memory for PD start");

    // Map
    if PAGING.lock().get_ptr(PD_START).is_some() {
        panic!("PD_START ({:#x}) is already mapped?!?!", PD_START);
    }

    let map = Paging::newmap(PD_START, phys);
    PAGING.lock().map(map).expect("could not map PD start");

    // That page only contains two u64s:
    // a pointer to get()
    let ptr = PD_START as *mut u64;
    unsafe {
        *ptr = get as u64;
    }
    // and a pointer to set()
    let ptr = unsafe { ptr.add(1) };
    unsafe {
        *ptr = set as u64;
    }

    // Now fill some values
    set3("print", crate::term::_print_str as u64);
    set3("yld", crate::tasks::scheduler::schedule as u64);
    set3(
        "ioapic_redirect_irq",
        crate::drivers::apic::ioapic::_set_irq_redirection as u64,
    );
    set3("unmask", crate::drivers::apic::ioapic::_unmask as u64);
}

pub extern "C" fn get(strptr: u64, sz: usize) -> u64 {
    let name = strptr2str(strptr, sz);
    if name.is_err() {
        return 0;
    }
    let name = String::from(name.unwrap());
    get2(&name)
}

pub fn get2(name: &String) -> u64 {
    let lock = PD.lock();
    let result = lock.get(name);
    match result {
        Some(k) => *k,
        None => 0,
    }
}

/*pub fn get3(name: &str) -> u64 {
    get2(&String::from(name))
}*/

pub extern "C" fn set(strptr: u64, sz: usize, ptr: u64) {
    let name = strptr2str(strptr, sz);
    if name.is_err() {
        return;
    }
    let name = String::from(name.unwrap());
    set2(&name, ptr);
}

pub fn set2(name: &String, ptr: u64) {
    PD.lock().insert(String::clone(name), ptr);
}

pub fn set3(name: &str, ptr: u64) {
    set2(&String::from(name), ptr);
}
