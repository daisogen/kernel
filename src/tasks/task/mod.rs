pub mod regs;

use core::arch::global_asm;
use regs::SavedState;

pub struct Task {
    pub state: SavedState,
    pub rip: u64,
    pub rsp: u64,
}

impl Task {
    pub fn new() -> Task {
        Task {
            state: SavedState::new(),
            rip: 0,
            rsp: 0,
        }
    }

    /*pub fn dispatch(&self) -> ! {
        unsafe {
            dispatch(&self.state as *const SavedState, self.rip, self.rsp);
        }
    }*/

    pub fn dispatch_saving(&self) {
        unsafe {
            dispatch_saving(&self.state as *const SavedState, self.rip, self.rsp);
        }
    }
}

global_asm!(include_str!("dispatcher.s"));

extern "C" {
    //fn dispatch(ss: *const SavedState, rip: u64, rsp: u64) -> !;
    fn dispatch_saving(ss: *const SavedState, rip: u64, rsp: u64);
    //fn restore_kernel_state() -> !;
}

/*pub fn wrapper_restore_kernel_state() -> ! {
    // Only because of RKS not being available outside of the module
    unsafe {
        restore_kernel_state();
    }
}*/
