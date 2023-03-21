use super::consts::*;
use crate::acpi;
use crate::mem::paging;
use alloc::{vec, vec::Vec};
use spin::Mutex;

// This should be a OnceLock but I can't get it to work :/
static ID: Mutex<u8> = Mutex::new(0);
static ADDRESS: Mutex<u64> = Mutex::new(0);
static GSIBASE: Mutex<u32> = Mutex::new(0);

static ISOS: Mutex<Vec<&MADT_ISO>> = Mutex::new(vec![]);

// irq -> (gsi, high/low, edge/level)
fn irq2gsi(irq: u8) -> (u32, bool, bool) {
    for i in &*ISOS.lock() {
        if i.irq == irq {
            return (i.gsi, i.flags & 2 != 0, i.flags & 8 != 0);
        }
    }

    (irq as u32, true, true)
}

pub fn init() {
    if *ADDRESS.lock() != 0 {
        // Already initialized
        return;
    }

    // Where's the IOAPIC? MADT, the ACPI table, knows
    let header = acpi::get("APIC").expect("no MADT in ACPI");
    let length = header.length as u64;

    let base = header as *const acpi::SDTHeader as u64;
    let mut off = MADT_RECORDS_START;
    let mut ncores = 0; // Let's count that too
    while off < length {
        let entry_type = unsafe { *((base + off) as *const u8) };
        off += 1;
        let record_length = unsafe { *((base + off) as *const u8) };
        off += 1;

        match entry_type {
            0 => {
                ncores += 1;
            }
            1 => {
                // IOAPIC
                if *ADDRESS.lock() != 0 {
                    todo!("multiple IOAPICs");
                }

                let ioapic = unsafe { &*((base + off) as *const MADT_IOAPIC) };
                *ID.lock() = ioapic.id;
                *ADDRESS.lock() = ioapic.address as u64;
                *GSIBASE.lock() = ioapic.gsibase;
                assert_eq!(*GSIBASE.lock(), 0);
            }
            2 => {
                // Interrupt Source Override
                let iso = unsafe { &*((base + off) as *const MADT_ISO) };
                (*ISOS.lock()).push(iso);
            }
            _ => {}
        }

        off += record_length as u64 - 2;
    }

    // Simple map of one page
    let addr = *(ADDRESS.lock()) as u64;
    let mut map = paging::Paging::newmap(addr, addr);
    map.nx = true;
    map.pcd = true;
    paging::PAGING.lock().map(map).unwrap();

    // Save this
    *crate::utils::NCORES.lock() = ncores;
}

// ---

fn read_reg(off: u8) -> u32 {
    unsafe {
        *((*ADDRESS.lock() + IOREGSEL) as *mut u32) = off as u32;
        *((*ADDRESS.lock() + IOWIN) as *const u32)
    }
}

fn write_reg(off: u8, val: u32) {
    unsafe {
        *((*ADDRESS.lock() + IOREGSEL) as *mut u32) = off as u32;
        *((*ADDRESS.lock() + IOWIN) as *mut u32) = val;
    }
}

fn read_redirection(gsi: u32) -> u64 {
    let reg: u8 = ((gsi - *GSIBASE.lock()) * 2 + 16) as u8;
    let ret = read_reg(reg) as u64;
    let ret = ret | ((read_reg(reg + 1) as u64) << 32);
    ret
}

fn write_redirection(gsi: u32, val: u64) {
    let reg: u8 = ((gsi - *GSIBASE.lock()) * 2 + 16) as u8;
    write_reg(reg, val as u32);
    write_reg(reg + 1, (val >> 32) as u32);
}

fn set_irq_redirection(lapic_id: u32, vec: u8, irq: u8) {
    let (gsi, low, level) = irq2gsi(irq);

    let mut flags: u64 = MASKED;
    if low {
        flags |= LOW_POLARITY;
    }
    if level {
        flags |= TRIGGER_MODE;
    }

    let entry: u64 = (vec as u64) | flags | ((lapic_id as u64) << LID_SHIFT);
    write_redirection(gsi, entry);
}

// FFI version
pub extern "C" fn _set_irq_redirection(lapic_id: usize, vec: usize, irq: usize) {
    set_irq_redirection(lapic_id as u32, vec as u8, irq as u8);
}

/*fn mask(gsi: u32) {
    write_redirection(gsi, read_redirection(gsi) | MASKED);
}*/

fn unmask(gsi: u32) {
    write_redirection(gsi, read_redirection(gsi) & !MASKED);
}

// FFI version
pub extern "C" fn _unmask(gsi: u64) {
    unmask(gsi as u32);
}
