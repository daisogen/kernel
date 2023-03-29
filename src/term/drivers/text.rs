// For VGA text mode

use super::writer::Writer;
use crate::term::Color;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: ColorCode,
}
const SC_SIZE: usize = core::mem::size_of::<ScreenChar>();

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
pub const FB_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH * SC_SIZE;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct TextWriter {
    row: usize,
    col: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer for TextWriter {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.line_break(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.line_break();
                }

                self.buffer.chars[self.row][self.col] = ScreenChar {
                    ascii: byte,
                    color: self.color,
                };
                self.col += 1;
            }
        }
    }

    fn line_break(&mut self) {
        self.row += 1;
        self.col = 0;
        if self.row >= BUFFER_HEIGHT {
            self.scroll();
        }
    }

    fn clear_row(&mut self, row: usize) {
        // Maybe just memeset 0, ignoring color?
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = ScreenChar {
                ascii: b' ',
                color: self.color,
            };
        }
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col] = self.buffer.chars[row][col];
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.col = 0;
        self.row -= 1;
    }

    fn clear(&mut self) {
        // Clear all rows and move cursor to the top
        for i in 0..BUFFER_HEIGHT {
            self.clear_row(i);
        }

        self.col = 0;
        self.row = 0;
    }

    fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = ColorCode::new(fg, bg);
    }
}

use core::fmt;
impl fmt::Write for TextWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref TEXT_WRITER: Mutex<TextWriter> = Mutex::new(TextWriter {
        row: 0,
        col: 0,
        color: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}
