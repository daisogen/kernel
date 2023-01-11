// Constants

pub const MEMORY_MAP_ID: u64 = 0x2187f79e8612de07;
pub const MODULES_ID: u64 = 0x4b6fe466aade04ce;

// ---

// Bootloader to kernel

#[repr(C, packed)]
pub struct StivaleStruct {
    bootloader_brand: [u8; 64],
    bootloader_version: [u8; 64],
    pub tag: *const StivaleTag,
}

#[repr(C, packed)]
pub struct StivaleTag {
    pub id: u64,
    pub next: *const StivaleTag,
}

#[repr(C, packed)]
pub struct StivaleMMTag {
    pub tag: StivaleTag,
    pub entries: u64,
    // entries go here
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StivaleMMEntry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u32,
    pub unused: u32,
}

#[repr(C, packed)]
pub struct StivaleModulesTag {
    pub tag: StivaleTag,
    pub count: u64,
    // array here
}

#[repr(C, packed)]
pub struct StivaleModule {
    pub begin: u64,
    pub end: u64,
    pub string: [u8; 128],
}

pub const STIVALE2_MMAP_USABLE: u32 = 1;
pub const STIVALE2_MMAP_BOOTLOADER_RECLAIMABLE: u32 = 0x1000;
pub const STIVALE2_MMAP_KERNEL_AND_MODULES: u32 = 0x1001;

// ---

// Kernel to bootloader

#[repr(C, packed)]
pub struct StivaleHeader {
    pub entry_point: u64,
    pub stack: *const u8,
    pub flags: u64,
    pub tags: *const StivaleTag,
}

unsafe impl Sync for StivaleHeader {}
unsafe impl Sync for StivaleTag {}
