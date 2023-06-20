pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    IO(std::io::Error),

    UnknownFormatVersion(u8),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
