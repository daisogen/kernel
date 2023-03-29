// These are just some random things that I need, and putting
//   them in src/ feels wrong

pub mod cpu;
pub mod regs;

use alloc::str::Utf8Error;
use spin::Once;

// Source: https://stackoverflow.com/a/42067321/4900881
pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, alloc::str::Utf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    alloc::str::from_utf8(&utf8_src[0..nul_range_end])
}

pub fn strptr2str(strptr: u64, sz: usize) -> Result<&'static str, Utf8Error> {
    let name = strptr as *const u8;
    let name: &[u8] = unsafe { core::slice::from_raw_parts(name, sz) };
    alloc::str::from_utf8(name)
}

// This value is set by IOAPIC initialization
pub static NCORES: Once<usize> = Once::new();
pub fn ncores() -> usize {
    *NCORES.get().unwrap()
}

pub fn whoami() -> usize {
    (unsafe { core::arch::x86_64::__cpuid(0x01) }.ebx >> 24) as usize
}
