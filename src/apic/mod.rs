mod consts;

use crate::mem::paging::PAGING;
use crate::utils::regs;
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
    return unsafe { *((consts::ADDRESS + off) as *const u32) };
}

fn write_reg(off: u64, val: u32) {
    unsafe {
        *((consts::ADDRESS + off) as *mut u32) = val;
    }
}

static MAPPED: Mutex<bool> = Mutex::new(false);
fn enable_lapic() {
    let base = regs::rdmsr(consts::BASE_MSR);
    let base = base | consts::LAPIC_ENABLE;

    // Get APIC base address and map it to virtual memory
    let address = crate::page!(base);
    assert_eq!(address, consts::ADDRESS, "APIC: weird address");
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
    regs::wrmsr(consts::BASE_MSR, base);

    // Set SIVR bit 8 to start receiving interrupts
    let sivr: u32 = read_reg(consts::SIVR_OFFSET);
    let sivr = sivr | (1 << 8);
    write_reg(consts::SIVR_OFFSET, sivr);
}

pub fn init() {
    if !is_supported() {
        panic!("APIC: not supported");
    }
    disable_pic();
    enable_lapic();
    // Init IOAPIC here
}
