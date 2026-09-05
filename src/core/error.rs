use std::{error, fmt};

/// An error.
#[derive(Debug)]
pub struct Error {
    /// The error code.
    pub code: Option<isize>,
    /// The error message.
    pub message: Option<String>,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match (self.code, &self.message) {
            (Some(code), Some(message)) => write!(formatter, "{message} (code {code})"),
            (Some(code), _) => write!(formatter, "a LazyTODO error (code {code})"),
            (_, Some(message)) => message.fmt(formatter),
            _ => write!(formatter, "a LazyTODO error"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.message {
            Some(ref message) => message,
            _ => "an LazyTODO error",
        }
    }
}
