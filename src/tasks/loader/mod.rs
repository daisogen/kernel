mod errors;
mod parse;

use super::{PID, TASKS};
use crate::mem::paging::{Paging, PAGING};
use crate::mem::pmm;
use alloc::boxed::Box;
use core::error;
use errors::LoaderError;

pub fn load(addr: u64, size: usize) -> Result<PID, Box<dyn error::Error>> {
    let info = parse::parse(addr, size)?;
    let pid = super::alloc_pid();
    let base = super::pid_to_base(pid);

    // Map pages to base
    for (virt, phys) in info.pages.iter() {
        let virt = virt + base;
        let map = Paging::newmap(virt, *phys);
        let map = PAGING.lock().map(map);
        if map.is_err() {
            // Oops
            for (_, v) in info.pages.iter() {
                pmm::free(*v, 1);
            }

            return Err(Box::new(LoaderError::OOM));
        }
    }

    let task = &mut TASKS.lock()[pid];
    task.rip = base + info.entry; // Entry point
    task.rsp = base + (2 << 30); // +2GB

    Ok(pid)
}
