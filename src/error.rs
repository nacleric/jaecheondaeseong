mod io;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Error: failed to insert grapheme {}", 0)]
    FailedToInsertGrapheme(char),
    // #[error("Error: Invalid `Position` specified {}", 0)]
    // InvalidPosition(crate::CursorPosition),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::FailedToInsertGrapheme(c)
    }
}
