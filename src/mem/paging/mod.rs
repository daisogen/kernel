pub mod paging;
pub mod the;

pub use paging::Paging;
pub use the::*;

pub extern "C" fn phys_of(ptr: usize) -> usize {
    the::PAGING.lock().get_ptr(ptr as u64).unwrap_or_default() as usize
}
