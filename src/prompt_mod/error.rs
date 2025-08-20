use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse { ty: &'static str, cause: String },
    EmptyNotAllowed,
    RetriesExceeded,
    Validation(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Parse { ty, cause } => write!(f, "Failed to parse as {ty}: {cause}"),
            Error::EmptyNotAllowed => write!(f, "Empty input (no default provided)"),
            Error::RetriesExceeded => write!(f, "Maximum retry attempts exceeded"),
            Error::Validation(msg) => write!(f, "Validation failed: {msg}"),
        }
    }
}

impl std::error::Error for Error {}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
