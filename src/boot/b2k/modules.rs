// By the time the following is executed, there is now an allocator

use super::find_tag;
use crate::boot::structures::*;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

// This is the prettier (and sortable!) version of StivaleModule
#[derive(Eq)]
pub struct Module {
    pub id: usize,
    pub name: String,
    pub begin: u64,
    pub end: u64,
}

impl core::cmp::Ord for Module {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl core::cmp::PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub static MODULES: Mutex<Vec<Module>> = Mutex::new(Vec::new());

pub fn parse_modules(boot_info: &StivaleStruct) {
    let ptr = find_tag(boot_info, MODULES_ID).expect("no modules tag from bootloader");
    let ptr = ptr as *const StivaleModulesTag;
    let tag = unsafe { &*ptr };
    let count = tag.count as usize;

    let ptr = ptr as usize;
    let ptr = ptr + core::mem::size_of::<StivaleModulesTag>();
    let mut ptr = ptr as *const StivaleModule;

    let mut modules = MODULES.lock();
    for _ in 0..count {
        let module: &StivaleModule = unsafe { &*ptr };
        let sid = crate::utils::str_from_u8_nul_utf8(&module.string);
        let sid = sid.expect("one of the modules has not a valid UTF-8 string");

        // sid = <id> <space> <name>
        let mut split = sid.split(' ');
        let ids = split.next().expect("invalid module string");
        let id = ids.parse::<usize>();
        let id = id.expect(&format!("module {} is not a number", ids));
        let name = split
            .next()
            .expect(&format!("module string for {} has no name", id));

        modules.push(Module {
            id: id,
            name: String::from(name),
            begin: module.begin,
            end: module.end,
        });

        ptr = unsafe { ptr.add(1) };
    }

    // Now we sort
    modules.sort();
}
