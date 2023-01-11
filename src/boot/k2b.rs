// Kernel to bootloader

use super::structures::*;
use crate::mem::PAGE_SIZE;

#[repr(C, align(4096))]
struct PageAligned<T>(T);

// The stack that will be loaded by the bootloader
pub const STACK_SIZE: usize = 4 * PAGE_SIZE; // One page was way too small
static STACK: PageAligned<[u8; STACK_SIZE]> = PageAligned([0; STACK_SIZE]);

pub fn get_stack_addr() -> u64 {
    STACK.0.as_ptr_range().start as u64
}

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
