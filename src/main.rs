#![no_std]
#![no_main]
#![feature(rustc_private)]

#[macro_use]
extern crate bitfield;

extern crate compiler_builtins;

mod boot;
mod desc;
mod mem;
mod panic;
mod term;

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
    println!("[OK]");

    loop {}
}
