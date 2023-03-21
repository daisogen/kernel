pub const BASE_MSR: u32 = 0x1B;
pub const LAPIC_ENABLE: u64 = 1 << 11;
pub const ADDRESS: u64 = 0xFEE00000;
pub const EOI_OFFSET: u64 = 0xB0;
pub const SIVR_OFFSET: u64 = 0xF0;

// --- IOAPIC ---

pub const MADT_RECORDS_START: u64 = 0x2C;
pub const IOREGSEL: u64 = 0;
pub const IOWIN: u64 = 0x10;

pub const LOW_POLARITY: u64 = 1 << 13;
pub const TRIGGER_MODE: u64 = 1 << 15;
pub const MASKED: u64 = 1 << 16;
pub const LID_SHIFT: u64 = 56;

#[repr(C, packed)]
pub struct MADT_IOAPIC {
    pub id: u8,
    pub reserved: u8,
    pub address: u32,
    pub gsibase: u32,
}

#[repr(C, packed)]
pub struct MADT_ISO {
    pub bus: u8,
    pub irq: u8,
    pub gsi: u32,
    pub flags: u16,
}
