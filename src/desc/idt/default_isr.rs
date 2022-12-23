#[no_mangle]
pub extern "C" fn default_isr(v: u64) -> ! {
    panic!("Unexpected interrupt: {:#x}", v);
}
