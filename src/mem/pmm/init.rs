use super::ops::calloc;
use super::Frame;
use super::FrameArr;
use crate::boot;
use crate::mem::PAGE_SIZE;
use crate::npages;

pub fn init() {
    let nentries = unsafe { boot::MM_NENTRIES };
    let entries = unsafe { &boot::MM_ENTRIES };

    let mut first: *const Frame = core::ptr::null();

    for i in 0..nentries {
        let base = entries[i].base;
        let length = entries[i].length;
        let entry_type = entries[i].entry_type;
        match entry_type {
            boot::STIVALE2_MMAP_USABLE => {
                if length < (2 * PAGE_SIZE) as u64 {
                    continue;
                }
            }
            _ => continue,
        }

        if first == core::ptr::null() {
            first = base as *const Frame;
        }

        super::STATE.lock().nregions += 1;
        init_region(base, length);
    }

    if first.is_null() {
        panic!("No available regions");
    }

    // Let's get an array of regions
    let nregions = super::STATE.lock().nregions;
    let needed = npages!(nregions * core::mem::size_of::<*const Frame>());

    // Set the first region for now, in the stack
    // This is *very* unsafe but it's the simplest way to do it
    super::STATE.lock().regions = FrameArr(&mut first as *mut *const Frame);

    // Now we have a valid state so we can allocate the first page
    super::STATE.lock().regions = FrameArr(calloc(needed).unwrap() as *mut *const Frame);
    // And now we're safe, no stack shenanigans
    // It's time to fill the regions for good, so loop again
    let mut ptr = super::STATE.lock().regions;
    for i in 0..nentries {
        let base = entries[i].base;
        let length = entries[i].length;
        let entry_type = entries[i].entry_type;
        match entry_type {
            boot::STIVALE2_MMAP_USABLE => {
                if length < (2 * PAGE_SIZE) as u64 {
                    continue;
                }
            }
            _ => continue,
        }

        // Fill it up
        unsafe {
            *(ptr.0) = base as *const Frame;
            ptr.0 = ptr.0.add(1);
        }
    }
}

fn init_region(base: u64, length: u64) {
    let npages = length as usize / PAGE_SIZE;
    let bmap_size = (npages + 8 - 1) / 8;
    // How many pages for metadata?
    let needed = core::mem::size_of::<Frame>() + bmap_size;
    let needed = npages!(needed);

    // That's it, format the region
    let frame: &mut Frame = unsafe { &mut *(base as *mut Frame) };
    frame.first = base + (needed * PAGE_SIZE) as u64;
    frame.pages = npages - needed;

    // Clean the bitmap
    let bmap: *mut u8 = (base + super::SZFRAME as u64) as *mut u8;
    unsafe {
        compiler_builtins::mem::memset(bmap, 0, bmap_size);
    }
}
