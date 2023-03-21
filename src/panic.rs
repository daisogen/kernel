use crate::println;
use crate::term;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: halt all threads
    crate::utils::regs::cli();
    term::set_color(term::Color::White, term::Color::Red);
    println!("Guru Meditation\n{}", info);
    loop {}
}
