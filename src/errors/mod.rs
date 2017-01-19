use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum ReadError {
    StdIoError(io::Error),
    UnknownFormat(Vec<u8>),
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError::StdIoError(e)
    }
}

#[derive(Debug)]
pub enum ExtractError {
    StdIoError(io::Error),
    InvalidFilePath(String)
}

impl From<io::Error> for ExtractError {
    fn from(e: io::Error) -> Self {
        ExtractError::StdIoError(e)
    }
}

#[derive(Debug)]
pub enum Error {
    ReadError(ReadError),
    ExtractError(ExtractError),
}

impl From<ReadError> for Error {
    fn from(e: ReadError) -> Self {
        Error::ReadError(e)
    }
}

impl From<ExtractError> for Error {
    fn from(e: ExtractError) -> Self {
        Error::ExtractError(e)
    }
}