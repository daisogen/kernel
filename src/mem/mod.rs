pub mod alloc;
pub mod paging;
pub mod pmm;

pub const PAGE_SIZE: usize = 1 << 12;
pub const HIGHER_HALF: u64 = 0xFFFF800000000000;
pub const HH: u64 = HIGHER_HALF;

#[macro_export]
macro_rules! npages {
    ($bytes:expr) => {
        ($bytes + PAGE_SIZE - 1) / PAGE_SIZE
    };
}

#[macro_export]
macro_rules! page {
    ($addr:expr) => {
        $addr & !0xFFF
    };
}

#[macro_export]
macro_rules! pageoff {
    ($addr:expr) => {
        $addr & 0xFFF
    };
}
