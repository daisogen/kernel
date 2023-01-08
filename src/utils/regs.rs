// Some functions to access _weird_ registers

use core::arch::asm;

pub fn get_cr2() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {x}, cr2",
            x = out(reg) x,
            options(nostack, preserves_flags));
    }
    x
}

pub fn set_cr3(cr3: u64) {
    unsafe {
        asm!("mov cr3, {x}",
             x = in(reg) cr3,
             options(nostack, preserves_flags));
    }
}

/*pub fn get_cr4() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {x}, cr4",
             x = out(reg) x,
             options(nostack, preserves_flags));
    }
    x
}

pub fn set_cr4(cr4: u64) {
    unsafe {
        asm!("mov cr4, {x}",
             x = in(reg) cr4,
             options(nostack, preserves_flags));
    }
}*/

pub fn rdmsr(msr: u32) -> u64 {
    let mut lsbb: u64;
    let mut msbb: u64;
    unsafe {
        asm!("rdmsr",
             in("ecx") msr,
             out("edx") msbb,
             out("eax") lsbb,
             options(nostack, preserves_flags));
    }
    msbb << 32 | lsbb
}

pub fn wrmsr(msr: u32, v: u64) {
    let lsbb = v as u32;
    let msbb = (v >> 32) as u32;
    unsafe {
        asm!("wrmsr",
             in("ecx") msr,
             in("edx") msbb,
             in("eax") lsbb,
             options(nostack, preserves_flags));
    }
}
