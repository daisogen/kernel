use crate::mem::paging::{Paging, PAGING};
use crate::mem::pmm;
use crate::mem::HH;
use crate::page;
use crate::print;
use crate::println;
use crate::utils::cpu::IRETQ;
use core::arch::global_asm;

global_asm!(include_str!("asm.s"));

extern "C" {
    pub fn asm_pf();
}

pub fn get_asm_addr() -> u64 {
    asm_pf as u64
}

const PFERR: &str = "PWURI";

#[no_mangle]
pub extern "C" fn pf_isr(err: u64, rsp: u64) {
    let addr = crate::utils::regs::get_cr2();
    let iretq = (rsp + 16 * 8) as *const IRETQ;
    let iretq: &IRETQ = unsafe { &*iretq };
    let culprit = iretq.rip;

    if addr == culprit {
        panic!("Page fault on jmp: {:#x}", addr);
    }

    // Does this come from heap/stack regions?
    if addr >= HH {
        let gbi = (addr - HH) >> 30; // GB index
        if gbi % 2 == 1 {
            // GB index is odd, so surely heap/stack
            // Was it due to it not being present?
            if err & 1 != 0 {
                todo!("Present page, problem with perms? Kill?");
            }

            let phys = pmm::calloc(1);
            if phys.is_err() {
                // TODO: kill the program and stuff, right?
                todo!("OOM allocating for {:#x}. Oops.", addr);
            }

            let page = page!(addr);
            let map = Paging::newmap(page, phys.unwrap());
            PAGING
                .lock()
                .map(map)
                .expect("OOM allocating pages in PF handler (TODO)");

            return;
        }
    }

    // --- We messed up, time to go bald ---
    // This is a bit weird but there might not be an allocator yet, so
    //   I can't construct a string
    println!();
    print!("A kernel panic is coming. Page fault: [");
    for i in 0..5 {
        if err & (1 << i) != 0 {
            print!("{}", PFERR.chars().nth(i).unwrap());
        }
    }
    println!("]");

    panic!("Unhandled page fault for {:#x} by {:#x}", addr, culprit);
}
