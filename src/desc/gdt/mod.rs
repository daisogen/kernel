pub mod code;
pub mod data;

use core::arch::global_asm;

// No allocator yet, so fix the number of entries
const N_ENTRIES: usize = 1 + 2;
pub const KCODE: u16 = 0x08;
//pub const KDATA: u16 = 0x10;

#[repr(transparent)]
struct GDTstruct {
    entries: [u64; N_ENTRIES],
}

#[repr(C, packed)]
struct GDTRstruct {
    limit: u16,
    addr: *const GDTstruct,
}

static mut GDT: GDTstruct = GDTstruct {
    entries: [0; N_ENTRIES],
};
static mut GDTR: GDTRstruct = GDTRstruct {
    limit: (N_ENTRIES as u16 * 8) - 1,
    addr: unsafe { &GDT as *const GDTstruct },
};

global_asm!(include_str!("lgdt.s"));
extern "C" {
    fn switchGDT(gdtr: *const GDTRstruct) -> u64;
}

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
