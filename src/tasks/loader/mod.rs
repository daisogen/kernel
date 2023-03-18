mod parse;
mod reloc;

use super::PID;
use crate::mem::paging::{Paging, PAGING};
use crate::mem::pmm;

pub fn load(addr: u64, size: usize) -> anyhow::Result<PID> {
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

            anyhow::bail!("Out of memory");
        }
    }

    let task = super::get_mut_task(pid);
    task.rip = base + info.file.ehdr.e_entry; // Entry point
    task.rsp = base + (2 << 30); // +2GB

    Ok(pid)
}
