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
        if ptr.is_null() {
            return None;
        }
    }
}

pub fn get_rsdp(boot_info: &StivaleStruct) -> u64 {
    let rsdp = find_tag(boot_info, RSDP_ID);
    let rsdp = rsdp.expect("ACPI: no RSDP from bootloader");
    let rsdp = unsafe { &*(rsdp as *const StivaleRSDP) };
    rsdp.rsdp
}
