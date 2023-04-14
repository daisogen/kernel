mod color;
pub mod drivers;

use crate::utils::strptr2str;
pub use color::Color;
use core::fmt;
use drivers::text::TEXT_WRITER;
use drivers::writer::Writer;

const TEXT_MODE: bool = true;

// Rust's formatting macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::term::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    if TEXT_MODE {
        TEXT_WRITER.lock().write_fmt(args).unwrap();
    }
}

pub extern "C" fn _print_str(strptr: u64, size: usize) {
    let name = strptr2str(strptr, size);
    if name.is_err() {
        return;
    }
    print!("{}", name.unwrap());
}

// Custom formatting macro
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::println!("debug: {}", format_args!($($arg)*));
    }
}

// Abstractions
pub fn init() {
    if TEXT_MODE {
        TEXT_WRITER.lock().init();
    }
}

pub fn set_color(fg: Color, bg: Color) {
    if TEXT_MODE {
        TEXT_WRITER.lock().set_color(fg, bg);
    }
}

/*pub fn clear() {
    if TEXT_MODE {
        TEXT_WRITER.lock().clear();
    }
}*/
