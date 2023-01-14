#[derive(Debug)]
pub enum LoaderError {
    NoPHDRs,
    OOM,
    NoSections,
    EnigmaRelocation, // lmao
    OutOfRange,
    Unaligned,
}

impl core::error::Error for LoaderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match *self {
            LoaderError::NoPHDRs => None,
            LoaderError::OOM => None,
            LoaderError::NoSections => None,
            LoaderError::EnigmaRelocation => None,
            LoaderError::OutOfRange => None,
            LoaderError::Unaligned => None,
        }
    }
}

impl core::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            LoaderError::NoPHDRs => {
                write!(f, "No program headers")
            }
            LoaderError::OOM => {
                write!(f, "Out of memory")
            }
            LoaderError::NoSections => {
                write!(f, "No sections")
            }
            LoaderError::EnigmaRelocation => {
                write!(f, "Enigma relocation")
            }
            LoaderError::OutOfRange => {
                write!(f, "Out of range")
            }
            LoaderError::Unaligned => {
                write!(f, "Unaligned address")
            }
        }
    }
}
