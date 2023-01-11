use super::{HH, PAGE_SIZE};
use crate::mem::paging::{Paging, PAGING};
use crate::npages;
use core::arch::global_asm;
use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

// Second GB (HIGHER_HALF + 1GB --- HIGHER_HALF + 2GB)
// is for kernel heap and stack

pub fn init_heap() {
    // Nothing gets allocated here. It's done on demand by the PF handler.
    let heap_start: usize = (HH + (1 << 30)) as usize;
    let heap_size: usize = 1 << 29; // 512MB
    unsafe {
        ALLOCATOR.init(heap_start, heap_size);
    }

    // Mount stack pages
    let new_stack_base = HH + (2 << 30) - crate::boot::k2b::STACK_SIZE as u64;
    let mut virt = new_stack_base;
    let mut phys = PAGING
        .lock()
        .get_ptr(crate::boot::k2b::get_stack_addr())
        .unwrap();
    let npages = npages!(crate::boot::k2b::STACK_SIZE);
    for _ in 0..npages {
        let mut map = Paging::newmap(virt, phys);
        map.nx = true; // A little caution never killed nobody
        PAGING
            .lock()
            .map(map)
            .expect("OOM while mounting stack pages");

        virt += PAGE_SIZE as u64;
        phys += PAGE_SIZE as u64;
    }

    // And change stack
    unsafe {
        switch_stack(crate::boot::k2b::get_stack_addr(), new_stack_base);
    }
}

extern "C" {
    fn switch_stack(current: u64, new: u64) -> u64;
}

global_asm!(
    "
switch_stack:
    mov rax, rsp
    sub rax, rdi
    add rax, rsi
    mov rsp, rax
    ret
"
);
