mod consts;

use crate::utils::regs::out8;
use consts::*;

fn send_command(cmd: u8) {
    out8(CMD_PORT, cmd);
}

fn send_data(data: u8) {
    out8(CHANNEL0_DATA_PORT, data);
}

pub fn set_frequency(ms: u16) {
    let reload_value: u16 = (ms * INPUT_CLOCK_FREQUENCY) / 3000;
    send_command(ICW);
    send_data((reload_value & 0xFF) as u8);
    send_data((reload_value >> 8) as u8);
}

pub fn init() {
    todo!("PIT pending");
    crate::drivers::apic::ioapic::set_irq_redirection(0, 0x20, PIT_IRQ);
}
