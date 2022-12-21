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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
    }

    fn clear(&mut self) {
        todo!();
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
