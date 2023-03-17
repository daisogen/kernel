pub mod code;
pub mod data;
pub mod tss;

use core::arch::global_asm;
use spin::Mutex;

// No allocator yet, so fix the number of entries
const BASIC_ENTRIES: usize = 1 + 2; // Null, KCODE and KDATA
const MAX_ENTRIES: usize = BASIC_ENTRIES + 2 * crate::MAX_CORES; // All TSS
pub const KCODE: u16 = 0x08;
//pub const KDATA: u16 = 0x10;

#[repr(transparent)]
struct GDTstruct {
    entries: [u64; MAX_ENTRIES],
}

#[repr(C, packed)]
struct GDTRstruct {
    limit: u16,
    addr: *const GDTstruct,
}

global_asm!(include_str!("lgdt.s"));
extern "C" {
    fn switchGDT(gdtr: *const GDTRstruct) -> u64;
}

// ---

static mut GDT: GDTstruct = GDTstruct {
    entries: [0; MAX_ENTRIES],
};
static mut GDTR: GDTRstruct = GDTRstruct {
    limit: (BASIC_ENTRIES as u16 * 8) - 1, // So far, only basic entries
    addr: unsafe { &GDT as *const GDTstruct },
};

// This is now specific for Daisōgen
pub fn init() {
    unsafe {
        GDT.entries[1] = code::CodeGDTEntry {
            conforming: false,
            dpl: 0,
            avl: false,
        }
        .real()
        .raw();

        GDT.entries[2] = data::DataGDTEntry {}.real().raw();
    }

    // Load
    unsafe {
        switchGDT(&GDTR);
    }
}

static USED_ENTRIES: Mutex<usize> = Mutex::new(3);
pub fn add_tss(ptr: u64) -> u16 {
    let mut locked = USED_ENTRIES.lock();
    let id = *locked;
    *locked += 2;

    let entry = tss::TSSGDTEntry { ptr };
    unsafe {
        (GDT.entries[id], GDT.entries[id + 1]) = entry.real().raw();
        GDTR.limit = (*locked as u16 * 8) - 1;
        switchGDT(&GDTR);
    }

    // Return selector
    (id * 8) as u16
}
