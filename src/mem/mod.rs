pub mod paging;
pub mod pmm;

pub const PAGE_SIZE: usize = 1 << 12;

#[macro_export]
macro_rules! npages {
    ($bytes:expr) => {
        ($bytes + PAGE_SIZE - 1) / PAGE_SIZE
    };
}
