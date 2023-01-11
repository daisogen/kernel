pub mod memory_map;
pub mod modules;

use crate::boot::structures::*;
pub use memory_map::*;
pub use modules::*;

fn find_tag(boot_info: &StivaleStruct, theid: u64) -> Option<*const StivaleTag> {
    let mut ptr = boot_info.tag;
    loop {
        let tag = unsafe { &*ptr };
        let id = tag.id;
        if id == theid {
            return Some(ptr);
        }

        ptr = tag.next;
        if ptr == core::ptr::null() {
            return None;
        }
    }
}
