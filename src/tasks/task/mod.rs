pub mod futex;
pub mod regs;
pub mod tls;

use core::arch::global_asm;
use regs::SavedState;

pub struct Task {
    pub state: SavedState,
    pub rip: u64,
    pub rsp: u64,

    pub futexes: futex::Futexes,
    pub tls: tls::TLS,
}

impl Task {
    pub fn new() -> Task {
        Task {
            state: SavedState::new(),
            rip: 0,
            rsp: 0,

            futexes: futex::Futexes::new(),
            tls: tls::TLS::new(),
        }
    }

    fn mypid(&self) -> super::PID {
        super::base_to_pid(self.rsp)
    }

    // clone() would be here for starting new threads

    /*pub fn dispatch(&self) -> ! {
        unsafe {
            dispatch(&self.state as *const SavedState, self.rip, self.rsp);
        }
    }*/

    pub fn dispatch_saving(&self) {
        super::scheduler::RUNNING.lock()[crate::utils::whoami()] = self.mypid();
        unsafe {
            dispatch_saving(&self.state as *const SavedState, self.rip, self.rsp);
        }
    }
}

global_asm!(include_str!("dispatcher.s"));

extern "C" {
    //fn dispatch(ss: *const SavedState, rip: u64, rsp: u64) -> !;
    fn dispatch_saving(ss: *const SavedState, rip: u64, rsp: u64);
    pub fn try_restore_kernel_state() -> u64;
}

pub fn try_restore() {
    // Just because it's inaccessible from the outside
    unsafe {
        try_restore_kernel_state();
    }
}
