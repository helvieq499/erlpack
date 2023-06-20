pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    UTF8(std::str::Utf8Error),

    UnknownFormatVersion(u8),
    UnknownTermType(u8),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::UTF8(value)
    }
}
