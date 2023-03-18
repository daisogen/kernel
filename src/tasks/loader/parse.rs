use crate::mem::{pmm, PAGE_SIZE};
use crate::{npages, page, pageoff};
use elf::endian::AnyEndian;
use elf::ElfBytes;
use hashbrown::HashMap;

pub struct ELFInfo {
    pub pages: HashMap<u64, u64>,
    pub file: ElfBytes<'static, AnyEndian>,
    pub base: Option<u64>,
}

pub fn parse(addr: u64, size: usize) -> anyhow::Result<ELFInfo> {
    let ptr = addr as *const u8;
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(ptr, size) };

    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).map_err(anyhow::Error::msg)?;
    let Some(phdrs) = file.segments() else { anyhow::bail!("No program headers"); };

    let mut pages: HashMap<u64, u64> = HashMap::new();
    for i in phdrs {
        if i.p_type != elf::abi::PT_LOAD {
            continue;
        }

        let mut vaddr = i.p_vaddr;
        let mut off = i.p_offset;
        let mut rem = i.p_filesz as usize;

        let npages = page!(vaddr + i.p_memsz); // Final page
        let npages = npages - page!(vaddr); // Minus first page
        let npages = 1 + npages!(npages as usize);
        for _ in 0..npages {
            let page = page!(vaddr);
            let pageoff = pageoff!(vaddr);
            let free = PAGE_SIZE - pageoff as usize;

            // If there's not a page allocated, get one
            if !pages.contains_key(&page) {
                let phys = pmm::calloc(1);
                if phys.is_err() {
                    for (_, v) in pages.iter() {
                        pmm::free(*v, 1);
                    }

                    anyhow::bail!("Out of memory");
                }

                pages.insert(page, phys.unwrap());
            }

            // Now copy bytes
            if rem > 0 {
                let dst = *pages.get(&page).unwrap();
                let dst = (dst + pageoff) as *mut u8;
                let src = (addr + off) as *const u8;
                let n = core::cmp::min(rem, free);
                unsafe {
                    compiler_builtins::mem::memcpy(dst, src, n);
                }
                rem -= n;
                off += n as u64;
            }

            vaddr += free as u64;
        }
    }

    Ok(ELFInfo {
        pages,
        file,
        base: None,
    })
}
