
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error)
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}