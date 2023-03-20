mod structures;

use crate::mem::{paging, PAGE_SIZE};
use alloc::{vec, vec::Vec};
use spin::Mutex;
use structures::*;

macro_rules! u8checksum {
    ($expr:expr, $ty:ty) => {{
        let slice = unsafe {
            core::slice::from_raw_parts(
                $expr as *const $ty as *const u8,
                core::mem::size_of::<$ty>(),
            )
        };
        let mut checksum: usize = 0;
        for i in slice {
            checksum += *i as usize;
        }
        (checksum & 0xFF) as u8
    }};
}

fn map_sdtheader(ptr: u64) -> &'static SDTHeader {
    let hdrsize = core::mem::size_of::<SDTHeader>();
    // Map hdrsize bytes
    let mut map = paging::Paging::newmap(ptr, ptr);
    map.ro = true;
    map.nx = true;
    map.npages = crate::npages!(crate::pageoff!(ptr) as usize + hdrsize);
    paging::PAGING.lock().map(map).unwrap();

    let header = unsafe { &*(ptr as *const SDTHeader) };
    let fullsize = header.length as usize;
    // Map again, but fullsize bytes this time
    map.npages = crate::npages!(crate::pageoff!(ptr) as usize + fullsize);
    paging::PAGING.lock().map(map).unwrap();

    header
}

static RSDTPTRS: Mutex<Vec<&'static SDTHeader>> = Mutex::new(vec![]);

pub fn parse(boot_info: &crate::boot::structures::StivaleStruct) {
    // Grab RSDP from bootloader
    let rsdp = crate::boot::b2k::get_rsdp(boot_info);
    // And map it
    let mut map = paging::Paging::newmap(rsdp, rsdp);
    map.ro = true;
    map.nx = true;
    paging::PAGING.lock().map(map).unwrap();

    // Check signature
    let rsdp = unsafe { &*(rsdp as *const RSDPDescriptor) };
    assert_eq!(
        SIGNATURE.as_bytes(),
        rsdp.signature,
        "ACPI: invalid signature"
    );

    let rev = rsdp.revision;
    if rev == 0 {
        // ACPI v1.0
        assert_eq!(
            u8checksum!(rsdp, RSDPDescriptor),
            0,
            "ACPI: invalid checksum"
        );

        // Map RSDT
        let rsdt = rsdp.rsdt as u64;
        let header = map_sdtheader(rsdt);

        // Get entries
        let hdrsize = core::mem::size_of::<SDTHeader>();
        let entries = (header.length as usize - hdrsize) / 4;
        let mut ptr = rsdt + hdrsize as u64;
        let mut lock = RSDTPTRS.lock();
        for _ in 0..entries {
            let entry = unsafe { *(ptr as *const u32) };
            let header = map_sdtheader(entry as u64);
            lock.push(header);
            ptr += 4;
        }
    } else if rev == 2 {
        // ACPI >= v2.0
        let rsdp = unsafe { &*(rsdp as *const RSDPDescriptor as *const RSDPDescriptor20) };
        assert_eq!(
            u8checksum!(rsdp, RSDPDescriptor20),
            0,
            "ACPI: invalid checksum"
        );

        todo!("ACPI v2.0");
    } else {
        panic!("ACPI: invalid revision: {}", rev);
    }
}

pub fn get(signature: &str) -> Option<&'static SDTHeader> {
    let vals = RSDTPTRS.lock();
    for header in &*vals {
        if header.signature == signature.as_bytes() {
            return Some(header);
        }
    }

    None
}
