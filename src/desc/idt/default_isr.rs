use spin::Mutex;

static WRAPS: Mutex<[u64; 256]> = Mutex::new([0; 256]);

pub fn set_wrapped_isr(v: u8, addr: u64) {
    let mut lock = WRAPS.lock();
    lock[v as usize] = addr;
}

pub extern "C" fn _set_wrapped_isr(v: u64, addr: u64) {
    set_wrapped_isr(v as u8, addr);
}

pub fn get_wrapped_isr(v: u8) -> Option<u64> {
    let lock = WRAPS.lock();
    let ret = lock[v as usize];
    if ret == 0 {
        None
    } else {
        Some(ret)
    }
}

// ---

#[no_mangle]
pub extern "C" fn default_isr(v: u64) {
    let addr = get_wrapped_isr(v as u8).unwrap_or_else(|| {
        panic!("Unexpected interrupt: {:#x}", v);
    });

    unsafe {
        jmp0(addr);
    }

    // Was I doing something?
    let pid = crate::tasks::scheduler::core_running();
    if pid == 0 {
        // Not at all, let's schedule
        crate::tasks::scheduler::schedule();
    } else {
        // I was indeed doing something, so let's gracefully return
        crate::utils::regs::sti(); // <- I'm pretty sure this is wrong (TODO?)
    }
}

extern "C" {
    fn jmp0(ptr: u64) -> u64;
}

core::arch::global_asm!(
    "
jmp0: jmp rdi
"
);
