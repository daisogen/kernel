use super::paging::Paging;
use crate::boot;
use crate::mem::pmm;
use crate::mem::PAGE_SIZE;
use crate::npages;
use crate::utils::regs::{rdmsr, wrmsr};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PAGING: Mutex<Paging> = {
        let cr3 = pmm::calloc(1).expect("No memory to initialize kpaging!");
        Mutex::new(Paging::new(cr3))
    };
}

const EFER_ADDR: u32 = 0xC0000080;
const EFER_BIT_NX: u32 = 11;

pub fn init_kernel_paging() {
    // Enable NX on the EFER
    let efer = rdmsr(EFER_ADDR) | (1 << EFER_BIT_NX);
    wrmsr(EFER_ADDR, efer);

    // Map framebuffer
    let mut map = Paging::newmap(0xB8000, 0xB8000);
    map.npages = npages!(crate::term::drivers::text::FB_SIZE);
    PAGING
        .lock()
        .map(map)
        .expect("Couldn't map framebuffer (OOM!)");

    // Map kernel and modules
    let nentries = unsafe { boot::MM_NENTRIES };
    let entries = unsafe { boot::MM_ENTRIES };
    for i in 0..nentries {
        let base = entries[i].base;
        let length = entries[i].length as usize;
        let entry_type = entries[i].entry_type;
        // All the entry types used here are guaranteed to be page-aligned
        match entry_type {
            boot::STIVALE2_MMAP_USABLE | boot::STIVALE2_MMAP_BOOTLOADER_RECLAIMABLE => {
                // These must be mapped 1:1
                let mut map = Paging::newmap(base, base);
                map.npages = npages!(length);
                PAGING.lock().map(map)
            }
            boot::STIVALE2_MMAP_KERNEL_AND_MODULES => {
                // These go to higher half
                // They're at the right offset, 1 MB
                let virt = crate::mem::HIGHER_HALF + base;
                let mut map = Paging::newmap(virt, base);
                map.npages = npages!(length);
                PAGING.lock().map(map)
            }
            _ => Ok::<(), ()>(()),
        }
        .expect("Couldn't map (OOM!)");
    }

    // That's it, let's go
    PAGING.lock().load();
}
