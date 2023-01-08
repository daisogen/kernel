#![no_std]
#![no_main]
#![feature(rustc_private)]

#[macro_use]
extern crate bitfield;
extern crate good_memory_allocator;

extern crate alloc;
extern crate compiler_builtins;

mod boot;
mod desc;
mod mem;
mod panic;
mod term;
mod utils;

use boot::StivaleStruct;

#[no_mangle]
pub extern "C" fn kmain(boot_info: &'static StivaleStruct) -> ! {
    println!("Daisogen booting up\n");

    print!("GDT ");
    desc::gdt::init();
    println!("[OK]");

    print!("IDT ");
    desc::idt::init();
    println!("[OK]");

    print!("Parsing boot info ");
    boot::parse_mm(boot_info);
    println!("[OK]");

    print!("Discovering memory ");
    mem::pmm::init::init();
    mem::paging::init_kernel_paging();
    mem::alloc::init_heap();
    println!("[OK]");

    loop {}
}
