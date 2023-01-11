pub mod regs;

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
}
