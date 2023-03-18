use super::parse::ELFInfo;
use crate::{page, pageoff};
use elf::relocation::Rela;
use elf::section::SectionHeader;

pub fn solve_relocations(info: &ELFInfo) -> anyhow::Result<()> {
    // Look for REL(A) sections
    let Some(shts) = info.file.section_headers() else {
        anyhow::bail!("No sections");
    };

    for i in shts {
        let r = match i.sh_type {
            elf::abi::SHT_RELA => solve_rela(info, &i),
            _ => Ok(()),
        };
        r?;
    }

    Ok(())
}

fn solve_rela(info: &ELFInfo, sh: &SectionHeader) -> anyhow::Result<()> {
    let relas = info
        .file
        .section_data_as_relas(sh)
        .map_err(anyhow::Error::msg)?;
    for i in relas {
        let r: anyhow::Result<()> = match i.r_type {
            elf::abi::R_X86_64_RELATIVE => solve_r_x86_64_relative(info, &i),
            _ => Err(anyhow::anyhow!("Enigma relocation")),
        };
        r?;
    }

    Ok(())
}

fn solve_r_x86_64_relative(info: &ELFInfo, sh: &Rela) -> anyhow::Result<()> {
    // [offset] = base + addend
    let Some(phys) = info.pages.get(&page!(sh.r_offset)) else {
        anyhow::bail!("Out of range");
    };
    let phys = phys + pageoff!(sh.r_offset);

    if phys % 8 != 0 {
        anyhow::bail!("Unaligned address");
    }

    // It's possible to mess this up by having a really big addend,
    // think ~0, but this is Daisogen: it really doesn't matter
    unsafe {
        *(phys as *mut u64) = info.base.unwrap() + sh.r_addend as u64;
    }

    Ok(())
}
