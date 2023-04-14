#![no_std]
#![no_main]
#![feature(rustc_private)]
#![feature(error_in_core)]
#![feature(link_llvm_intrinsics)]

#[macro_use]
extern crate bitfield;

extern crate alloc;
extern crate compiler_builtins;

mod boot;
mod bootstrap;
mod desc;
mod drivers;
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
    term::init();
    println!("Daisogen booting up\n");

    // TODO: Move this to arch
    print!("Architectural init ");
    debug!("GDT");
    desc::gdt::init();
    debug!("IDT");
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
    debug!("ACPI parsing");
    drivers::acpi::parse(boot_info);
    debug!("APIC init");
    drivers::apic::init();
    debug!("TSS init");
    desc::tss::init();
    debug!("IDT init2");
    desc::idt::init2();
    pd::init();
    println!("[OK]");

    println!("Bootstrapping...");
    utils::regs::sti();
    boot::b2k::parse_modules(boot_info);
    bootstrap::run();
    // That does not return
}
