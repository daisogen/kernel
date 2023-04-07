use crate::utils::regs::{in8, out8};

const CONTROL: u16 = 0x3D4;
const DATA: u16 = 0x3D5;

const CURSOR_START: u8 = 0;
const CURSOR_END: u8 = 15;
pub fn enable() {
    out8(CONTROL, 0x0A);
    out8(DATA, (in8(DATA) & 0xC0) | CURSOR_START);
    out8(CONTROL, 0x0B);
    out8(DATA, (in8(DATA) & 0xE0) | CURSOR_END);
}

pub fn mov(x: usize, y: usize) {
    let pos = y * super::BUFFER_WIDTH + x;
    out8(CONTROL, 0x0F);
    out8(DATA, pos as u8);
    out8(CONTROL, 0x0E);
    out8(DATA, (pos >> 8) as u8);
}
