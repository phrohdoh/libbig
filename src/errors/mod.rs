use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ReadError {
    StdIoError(io::Error),
    UnknownArchiveFormat(Vec<u8>),
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError::StdIoError(e)
    }
}

#[derive(Debug)]
pub enum ExtractError {
    StdIoError(io::Error),
    InvalidPath(PathBuf),
}

impl From<io::Error> for ExtractError {
    fn from(e: io::Error) -> Self {
        ExtractError::StdIoError(e)
    }
}

pub enum CreateError {
    StdIoError(io::Error),
}