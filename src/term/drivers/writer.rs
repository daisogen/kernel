use crate::term::color::Color;

// Trait for terminal writers
// TODO: Please get rid of this useless abstraction

pub trait Writer {
    // Just implement these :)
    fn init(&mut self);
    fn update_cursor(&self);
    fn write_byte(&mut self, byte: u8);
    fn backspace(&mut self);
    fn line_break(&mut self);
    fn clear_row(&mut self, row: usize);
    fn scroll(&mut self);
    fn clear(&mut self);
    fn set_color(&mut self, fg: Color, bg: Color);

    // ---

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}
