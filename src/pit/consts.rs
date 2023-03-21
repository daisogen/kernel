const PIT_IRQ: u8 = 0;
const CHANNEL0_DATA_PORT: u16 = 0x40;
const CMD_PORT: u16 = 0x43;

const ICW: u16 = 0x36; // Channel 0, lobyte/hibyte, rate generator, binary
const INPUT_CLOCK_FREQUENCY: u32 = 3579545;
