// These are just some random things that I need, and putting
//   them in src/ feels wrong

pub mod regs;

// Source: https://stackoverflow.com/a/42067321/4900881
pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, alloc::str::Utf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    alloc::str::from_utf8(&utf8_src[0..nul_range_end])
}
