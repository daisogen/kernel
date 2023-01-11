#[derive(Debug)]
pub enum LoaderError {
    NoPHDRs,
    OOM,
}

impl core::error::Error for LoaderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match *self {
            LoaderError::NoPHDRs => None,
            LoaderError::OOM => None,
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
        }
    }
}
