#![no_std]
#![no_main]

#[macro_use]
extern crate bitfield;

mod boot;
mod desc;
mod panic;
mod term;

use boot::StivaleStruct;

#[no_mangle]
pub extern "C" fn kmain(_boot_info: &'static StivaleStruct) -> ! {
    println!("Daisogen booting up\n");

    print!("GDT ");
    desc::gdt::init();
    println!("[OK]");

    print!("IDT ");
    desc::idt::init();
    println!("[OK]");

    loop {}
}
