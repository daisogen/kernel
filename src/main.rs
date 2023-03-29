#![no_std]
#![no_main]
#![feature(rustc_private)]
#![feature(error_in_core)]

#[macro_use]
extern crate bitfield;

extern crate alloc;
extern crate compiler_builtins;

mod boot;
mod bootstrap;
mod desc;
mod drivers;
mod futex;
mod mem;
mod panic;
mod pd;
mod tasks;
mod term;
mod utils;

use boot::structures::StivaleStruct;

// This is an arbitrary value but one *must* be specified
const MAX_CORES: usize = 32;

#[no_mangle]
pub extern "C" fn kmain(boot_info: &'static StivaleStruct) -> ! {
    term::clear();
    println!("Daisogen booting up\n");

    print!("GDT ");
    desc::gdt::init();
    println!("[OK]");

    print!("IDT ");
    desc::idt::init();
    println!("[OK]");

    print!("Parsing boot info ");
    boot::b2k::parse_mm(boot_info);
    println!("[OK]");

    print!("Discovering memory ");
    mem::pmm::init::init();
    mem::paging::init_kernel_paging();
    mem::alloc::init_heap();
    // Dynamic memory is available now
    println!("[OK]");

    print!("Final preps ");
    drivers::acpi::parse(boot_info);
    drivers::apic::init();
    desc::tss::init();
    desc::idt::init2();
    pd::init();
    println!("[OK]");

    println!("Bootstrapping...");
    utils::regs::sti();
    boot::b2k::parse_modules(boot_info);
    bootstrap::run();
    // That does not return
}
