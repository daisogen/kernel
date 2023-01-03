// This PMM is inherited from The Strife Project, my previous OS:
// https://github.com/the-strife-project/kernel/tree/main/src/mem/PMM

mod bitmap;
pub mod init;
pub mod ops;

use spin::Mutex;

#[repr(C, packed)]
struct Frame {
    first: u64,   // First page after the frame
    pages: usize, // Number of pages after the frame
}
const SZFRAME: usize = core::mem::size_of::<Frame>();

#[derive(Clone, Copy)]
struct FrameArr(*mut *const Frame);
unsafe impl Send for FrameArr {}

struct State {
    nregions: usize,
    regions: FrameArr,
}
static STATE: Mutex<State> = Mutex::new(State {
    nregions: 0,
    regions: FrameArr(core::ptr::null::<*const Frame>() as *mut *const Frame),
});
