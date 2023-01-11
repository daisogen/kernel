// This is a little bit complex since there's no allocator yet, so no Vec

use super::find_tag;
use crate::boot::structures::*;
use crate::mem::PAGE_SIZE;

// One page of memory map entries. Hope we don't need more.
pub static mut MM_NENTRIES: usize = 0;
const MM_MAXENTRIES: usize = PAGE_SIZE / core::mem::size_of::<StivaleMMEntry>();
pub static mut MM_ENTRIES: [StivaleMMEntry; MM_MAXENTRIES] = [StivaleMMEntry {
    base: 0,
    length: 0,
    entry_type: 0,
    unused: 0,
}; MM_MAXENTRIES];

pub fn parse_mm(boot_info: &StivaleStruct) {
    let ptr = find_tag(boot_info, MEMORY_MAP_ID).expect("no memory map tag from bootloader");
    let ptr = ptr as *const StivaleMMTag;
    let tag = unsafe { &*ptr };
    let nentries = tag.entries as usize;
    if nentries > MM_MAXENTRIES {
        panic!("too many memory entries :(");
    }
    unsafe {
        MM_NENTRIES = nentries;
    }

    let ptr = ptr as usize;
    let ptr = ptr + core::mem::size_of::<StivaleMMTag>();
    let mut ptr = ptr as *const StivaleMMEntry;
    for i in 0..nentries {
        unsafe {
            MM_ENTRIES[i] = *ptr;
        }

        ptr = unsafe { ptr.add(1) };
    }
}
