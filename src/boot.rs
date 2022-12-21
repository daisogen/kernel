#[repr(C)]
pub struct StivaleStruct {
    bootloader_brand: [u8; 64],
    bootloader_version: [u8; 64],
    tags: u64,
}

// ---

#[repr(C, packed)]
struct StivaleHeader {
    entry_point: u64,
    stack: *const u8,
    flags: u64,
    tags: *const (),
}

unsafe impl Sync for StivaleHeader {}

#[repr(C, align(4096))]
struct P2Align12<T>(T);

// The stack that will be loaded by the bootloader
const PAGE_SIZE: usize = 4096;
const STACK_SIZE: usize = PAGE_SIZE;
static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

// stivale2 boot header
#[used]
#[link_section = ".stivale2hdr"]
static STIVALE_HDR: StivaleHeader = StivaleHeader {
    entry_point: 0,
    stack: STACK.0.as_ptr_range().end,
    flags: 0,
    tags: core::ptr::null(),
};
