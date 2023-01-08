use super::HH;
use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

// Second GB (HIGHER_HALF + 1GB --- HIGHER_HALF + 2GB)
// is for kernel heap and stack

pub fn init_heap() {
    // Nothing gets allocated here. It's done on demand by the PF handler.
    let heap_start: usize = (HH + (1 << 30)) as usize;
    let heap_size: usize = 1 << 29; // 512MB
    unsafe {
        ALLOCATOR.init(heap_start, heap_size);
    }
}
