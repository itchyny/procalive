use std::convert::From;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoCommand,
    IoError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::IoError
    }
}
