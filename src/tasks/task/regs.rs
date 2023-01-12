#[allow(dead_code)]
enum RFlags {
    CF,
    MBO,
    PF,
    MBZ0,
    AF,
    MBZ1,
    ZF,
    SF,
    TF,
    IF,
    DF,
    OF,
    IOPL0,
    IOPL1,
    NT,
    MBZ2,
    RF,
    VM,
    AC,
    VIF,
    VIP,
    ID,
}

// Just valid (with must be one = 1) and interrupts enabled
const BASIC_RFLAGS: u64 = (1 << RFlags::MBO as u64) | (1 << RFlags::IF as u64);

#[repr(C, packed)]
pub struct SavedState {
    rflags: u64,
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

impl SavedState {
    pub fn new() -> SavedState {
        SavedState {
            rflags: BASIC_RFLAGS,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }
}
