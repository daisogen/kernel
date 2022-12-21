#![no_std]
#![no_main]

mod boot;
mod panic;

use boot::StivaleStruct;

static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn kmain(_boot_info: &'static StivaleStruct) -> ! {
    let vga_buffer = 0xB8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xB;
        }
    }

    loop {}
}
