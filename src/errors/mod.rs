use std::io;

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