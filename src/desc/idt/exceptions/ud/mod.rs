use core::arch::global_asm;

global_asm!(include_str!("asm.s"));

extern "C" {
    pub fn asm_ud();
}

pub fn get_asm_addr() -> u64 {
    asm_ud as u64
}

#[no_mangle]
pub extern "C" fn ud_isr() {
    let addr = crate::utils::regs::get_cr2();
    panic!("Invalid opcode at {:#x}", addr);
}
