mod errors;
mod parse;
mod reloc;

use super::PID;
use crate::mem::paging::{Paging, PAGING};
use crate::mem::pmm;
use alloc::boxed::Box;
use core::error;
use errors::LoaderError;

pub fn load(addr: u64, size: usize) -> Result<PID, Box<dyn error::Error>> {
    let mut info = parse::parse(addr, size)?;
    let pid = super::alloc_pid();
    let base = super::pid_to_base(pid);
    info.base = Some(base);
    reloc::solve_relocations(&info)?;

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

    let task = super::get_mut_task(pid);
    task.rip = base + info.file.ehdr.e_entry; // Entry point
    task.rsp = base + (2 << 30); // +2GB

    Ok(pid)
}
