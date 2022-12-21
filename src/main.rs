#![no_std]
#![no_main]

mod boot;
mod panic;
mod term;

use boot::StivaleStruct;

#[no_mangle]
pub extern "C" fn kmain(_boot_info: &'static StivaleStruct) -> ! {
    println!("Hello world! println!() works :)");
    panic!("Panic works too");
}
