pub const PIT_IRQ: u8 = 0;
pub const CHANNEL0_DATA_PORT: u16 = 0x40;
pub const CMD_PORT: u16 = 0x43;

pub const ICW: u16 = 0x36; // Channel 0, lobyte/hibyte, rate generator, binary
pub const INPUT_CLOCK_FREQUENCY: u32 = 3579545;
