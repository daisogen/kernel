use crate::mem::{pmm, PAGE_SIZE};
use alloc::boxed::Box;
use core::arch::global_asm;

#[repr(C, packed)]
pub struct TSS {
    reserved0: u32,
    rsps: [u64; 3],
    reserved1: u64,
    ists: [u64; 7],
    reserved2: u64,
    reserved3: u16,
    iopb: u16,
}

pub const TSS_SIZE: usize = core::mem::size_of::<TSS>();

impl TSS {
    pub fn new() -> TSS {
        TSS {
            reserved0: 0,
            rsps: [0; 3],
            reserved1: 0,
            ists: [0; 7],
            reserved2: 0,
            reserved3: 0,
            iopb: TSS_SIZE as u16,
        }
    }
}

pub const IST_PF: usize = 1;
//pub const IST_DF: usize = 2;
const USED_ISTS: usize = 2;

pub fn init() {
    let ncores = crate::utils::ncores();
    if ncores > 1 {
        todo!("Check this out");
    }

    for _ in 0..ncores {
        // Allocate
        let mut tss: Box<TSS> = Box::new(TSS::new());
        // 16KB ISTs
        for i in 0..USED_ISTS {
            let stack = pmm::calloc(4);
            let stack = stack.expect("Not enough memory for ISTs");
            let stack = stack + (PAGE_SIZE * 4) as u64; // bottom
            tss.ists[i] = stack;
        }
        // Now we leak it
        let leak: &'static mut TSS = Box::leak(tss);
        let ptr: u64 = leak as *const TSS as u64;
        // Add it to the GDT
        let sel = crate::desc::gdt::add_tss(ptr);
        // And load it
        unsafe {
            /*
            This is wrong. It must be done by each core.
            */
            load_tss(sel);
        }
    }
}

extern "C" {
    fn load_tss(sel: u16) -> u16;
}

global_asm!(
    "
load_tss:
    mov ax, di
    ltr ax
    ret"
);
