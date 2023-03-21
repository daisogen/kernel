mod consts;
pub mod ioapic;

use crate::mem::paging::PAGING;
use crate::utils::regs;
use consts::*;
use spin::Mutex;

static PIC_DISABLED: Mutex<bool> = Mutex::new(false);
fn disable_pic() {
    let mut locked = PIC_DISABLED.lock();
    if !*locked {
        regs::outb(0xA1, 0xFF);
        regs::outb(0x21, 0xFF);
        *locked = true;
    }
}

fn is_supported() -> bool {
    // CPUID.01h:EDX[9] specifies whether CPU has built-in local APIC
    (unsafe { core::arch::x86_64::__cpuid(0x01) }.edx >> 9) & 1 == 1
}

fn read_reg(off: u64) -> u32 {
    return unsafe { *((ADDRESS + off) as *const u32) };
}

fn write_reg(off: u64, val: u32) {
    unsafe {
        *((ADDRESS + off) as *mut u32) = val;
    }
}

static MAPPED: Mutex<bool> = Mutex::new(false);
fn enable_lapic() {
    let base = regs::rdmsr(BASE_MSR);
    let base = base | LAPIC_ENABLE;

    // Get APIC base address and map it to virtual memory
    let address = crate::page!(base);
    assert_eq!(address, ADDRESS, "APIC: weird address");
    {
        let mut lock = MAPPED.lock();
        if !*lock && PAGING.lock().get_ptr(address).is_some() {
            panic!("APIC: address in use");
        }
        let mut map = crate::mem::paging::Paging::newmap(address, address);
        map.nx = true;
        PAGING.lock().map(map).unwrap();
        *lock = true;
    } // Artificial scope

    // Make sure it's on
    regs::wrmsr(BASE_MSR, base);

    // Set SIVR bit 8 to start receiving interrupts
    let sivr: u32 = read_reg(SIVR_OFFSET);
    let sivr = sivr | (1 << 8);
    write_reg(SIVR_OFFSET, sivr);
}

pub fn init() {
    if !is_supported() {
        panic!("APIC: not supported");
    }
    disable_pic();
    enable_lapic();
    ioapic::init();
}
