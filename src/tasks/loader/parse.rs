use super::errors::LoaderError;
use crate::mem::{pmm, PAGE_SIZE};
use crate::{npages, page, pageoff};
use alloc::boxed::Box;
use core::error;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use hashbrown::HashMap;

pub struct ELFInfo {
    pub pages: HashMap<u64, u64>,
    pub entry: u64,
}

pub fn parse(addr: u64, size: usize) -> Result<ELFInfo, Box<dyn error::Error>> {
    let ptr = addr as *const u8;
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(ptr, size) };

    let file = ElfBytes::<AnyEndian>::minimal_parse(slice)?;
    let phdrs = file.segments();
    if phdrs.is_none() {
        return Err(Box::new(LoaderError::NoPHDRs));
    }
    let phdrs = phdrs.unwrap();
    let entry = file.ehdr.e_entry;

    let mut pages: HashMap<u64, u64> = HashMap::new();
    for i in phdrs {
        if i.p_type != elf::abi::PT_LOAD {
            continue;
        }

        let mut vaddr = i.p_vaddr;
        let mut off = i.p_offset;
        let mut rem = i.p_filesz as usize;
        let npages = npages!(i.p_memsz as usize);
        for _ in 0..npages {
            let page = page!(vaddr);
            let pageoff = pageoff!(vaddr);

            // If there's not a page allocated, get one
            if !pages.contains_key(&page) {
                let phys = pmm::calloc(1);
                if phys.is_err() {
                    for (_, v) in pages.iter() {
                        pmm::free(*v, 1);
                    }

                    return Err(Box::new(LoaderError::OOM));
                }

                pages.insert(page, phys.unwrap());
            }

            // Now copy bytes
            if rem > 0 {
                let dst = *pages.get(&page).unwrap();
                let dst = (dst + pageoff) as *mut u8;
                let src = (addr + off) as *const u8;
                let n = core::cmp::min(rem, PAGE_SIZE - pageoff as usize);
                unsafe {
                    compiler_builtins::mem::memcpy(dst, src, n);
                }
                rem -= n;
                off += n as u64;
            }

            vaddr += PAGE_SIZE as u64;
        }
    }

    Ok(ELFInfo {
        pages: pages,
        entry: entry,
    })
}
