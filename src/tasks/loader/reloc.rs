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

// https://docs.rs/elf/0.7.2/src/elf/abi.rs.html#2594
fn solve_rela(info: &ELFInfo, sh: &SectionHeader) -> anyhow::Result<()> {
    let relas = info
        .file
        .section_data_as_relas(sh)
        .map_err(anyhow::Error::msg)?;
    for i in relas {
        let r = match i.r_type {
            elf::abi::R_X86_64_RELATIVE => solve_r_x86_64_ba(info, &i),
            elf::abi::R_X86_64_GLOB_DAT => solve_r_x86_64_s(info, &i),
            x => Err(anyhow::anyhow!("Enigma relocation: {:?}", x)),
        }?;

        // Get physical address of this relocation
        let Some(phys) = info.pages.get(&page!(i.r_offset)) else {
            anyhow::bail!("Out of range");
        };
        let phys = phys + pageoff!(i.r_offset);

        if phys % 8 != 0 {
            anyhow::bail!("Unaligned address");
        }

        // Now actually write
        unsafe {
            *(phys as *mut u64) = r;
        }
    }

    Ok(())
}

fn solve_r_x86_64_ba(info: &ELFInfo, rel: &Rela) -> anyhow::Result<u64> {
    // [offset] = base + addend
    Ok(info.base.unwrap() + rel.r_addend as u64)
}

fn solve_r_x86_64_s(info: &ELFInfo, rel: &Rela) -> anyhow::Result<u64> {
    // [offset] = value of symbol
    let symbols = info
        .file
        .dynamic_symbol_table()
        .map_err(anyhow::Error::msg)?;
    let Some(symbols) = symbols else {
        anyhow::bail!("No symbols");
    };
    let symbols = symbols.0; // Don't care about names (.1)
    let symbol = symbols
        .get(rel.r_sym as usize)
        .map_err(anyhow::Error::msg)?;
    Ok(symbol.st_value)
}
