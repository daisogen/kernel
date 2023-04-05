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

    // clone() would be here for starting new threads

    pub fn dispatch(&self) -> ! {
        unsafe {
            dispatch(&self.state as *const SavedState, self.rip, self.rsp);
        }
    }
}

extern "C" {
    fn dispatch(ss: *const SavedState, rip: u64, rsp: u64) -> !;
}

global_asm!(include_str!("dispatcher.s"));
