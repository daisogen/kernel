pub mod gp;
pub mod pf;
pub mod ud;

pub const EXCEPTION_UD: usize = 0x6;
pub const EXCEPTION_GP: usize = 0xD;
pub const EXCEPTION_PF: usize = 0xE;
