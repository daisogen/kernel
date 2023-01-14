use super::errors::LoaderError;
use super::parse::ELFInfo;
use crate::{page, pageoff};
use alloc::boxed::Box;
use core::error;
use elf::relocation::Rela;
use elf::section::SectionHeader;

pub fn solve_relocations(info: &ELFInfo) -> Result<(), Box<dyn error::Error>> {
    // Look for REL(A) sections
    let shts = info.file.section_headers();
    if shts.is_none() {
        return Err(Box::new(LoaderError::NoSections));
    }
    let shts = shts.unwrap();

    for i in shts {
        let r = match i.sh_type {
            elf::abi::SHT_RELA => solve_rela(info, &i),
            _ => Ok(()),
        };

        if r.is_err() {
            return r;
        }
    }

    Ok(())
}

fn solve_rela(info: &ELFInfo, sh: &SectionHeader) -> Result<(), Box<dyn error::Error>> {
    let relas = info.file.section_data_as_relas(sh)?;
    for i in relas {
        let r: Result<(), Box<dyn error::Error>> = match i.r_type {
            elf::abi::R_X86_64_RELATIVE => solve_r_x86_64_relative(info, &i),
            _ => Err(Box::new(LoaderError::EnigmaRelocation)),
        };
        r?;
    }

    Ok(())
}

fn solve_r_x86_64_relative(info: &ELFInfo, sh: &Rela) -> Result<(), Box<dyn error::Error>> {
    // [offset] = base + addend
    let phys = info.pages.get(&page!(sh.r_offset));
    if phys.is_none() {
        return Err(Box::new(LoaderError::OutOfRange));
    }
    let phys = phys.unwrap() + pageoff!(sh.r_offset);

    if phys % 8 != 0 {
        return Err(Box::new(LoaderError::Unaligned));
    }

    // It's possible to mess this up by having a really big addend,
    // think ~0, but this is Daisogen: it really doesn't matter
    unsafe {
        *(phys as *mut u64) = info.base.unwrap() + sh.r_addend as u64;
    }

    Ok(())
}
