use crate::mem::PAGE_SIZE;

// Constants

const MEMORY_MAP_ID: u64 = 0x2187f79e8612de07;

// ---

// Bootloader to kernel

#[repr(C, packed)]
pub struct StivaleStruct {
    bootloader_brand: [u8; 64],
    bootloader_version: [u8; 64],
    tag: *const StivaleTag,
}

#[repr(C, packed)]
struct StivaleTag {
    id: u64,
    next: *const StivaleTag,
}

#[repr(C, packed)]
struct StivaleMMTag {
    tag: StivaleTag,
    entries: u64,
    // entries go here
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StivaleMMEntry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u32,
    unused: u32,
}

pub const STIVALE2_MMAP_USABLE: u32 = 1;
pub const STIVALE2_MMAP_BOOTLOADER_RECLAIMABLE: u32 = 0x1000;
pub const STIVALE2_MMAP_KERNEL_AND_MODULES: u32 = 0x1001;

// ---

// Kernel to bootloader

#[repr(C, packed)]
struct StivaleHeader {
    entry_point: u64,
    stack: *const u8,
    flags: u64,
    tags: *const StivaleTag,
}

unsafe impl Sync for StivaleHeader {}
unsafe impl Sync for StivaleTag {}

#[repr(C, align(4096))]
struct P2Align12<T>(T);

// The stack that will be loaded by the bootloader
const STACK_SIZE: usize = 4 * PAGE_SIZE; // One page was way too small
static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

// Memory map structure tag
#[used]
static MM_TAG: StivaleTag = StivaleTag {
    id: MEMORY_MAP_ID,
    next: core::ptr::null(),
};

// stivale2 boot header
#[used]
#[link_section = ".stivale2hdr"]
static STIVALE_HDR: StivaleHeader = StivaleHeader {
    entry_point: 0,
    stack: STACK.0.as_ptr_range().end,
    flags: 0,
    tags: &MM_TAG,
};

// ---

// Parsing
// This is a little bit complex since there's no allocator yet, so no vector

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
    let mut ptr = boot_info.tag;
    loop {
        let tag = unsafe { &*ptr };
        let id = tag.id;
        if id == MEMORY_MAP_ID {
            break;
        }

        ptr = tag.next;
        if ptr == core::ptr::null() {
            panic!("no memory map tag from bootloader");
        }
    }

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
