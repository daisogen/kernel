use super::ops::calloc;
use super::Frame;
use super::FrameArr;
use crate::boot;
use crate::mem::PAGE_SIZE;
use crate::npages;

/*
This is the first entry in the memory map marked as usable. This is in static
memory so that a pointer to it has static lifetime, which is used later to
"hack" alloc() in order to allocate de first page, used later to store all the
entries.
*/
static mut FIRST: *const Frame = core::ptr::null();

pub fn init() {
    let nentries = unsafe { boot::b2k::MM_NENTRIES };
    let entries = unsafe { &boot::b2k::MM_ENTRIES };

    for i in 0..nentries {
        let base = entries[i].base;
        let length = entries[i].length;
        let entry_type = entries[i].entry_type;
        match entry_type {
            boot::structures::STIVALE2_MMAP_USABLE => {
                if length < (2 * PAGE_SIZE) as u64 {
                    continue;
                }
            }
            _ => continue,
        }

        if unsafe { FIRST }.is_null() {
            unsafe {
                FIRST = base as *const Frame;
            }
        }

        super::STATE.lock().nregions += 1;
        init_region(base, length);
    }

    if unsafe { FIRST }.is_null() {
        panic!("No available regions");
    }

    // Let's get an array of regions
    let nregions = super::STATE.lock().nregions;
    let needed = npages!(nregions * core::mem::size_of::<*const Frame>());

    // Set the first region for now
    super::STATE.lock().regions = unsafe { FrameArr(&mut FIRST as *mut *const Frame) };

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
            boot::structures::STIVALE2_MMAP_USABLE => {
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
