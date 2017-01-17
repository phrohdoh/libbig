#![feature(slice_patterns)]

extern crate byteorder;

use std::io::{BufReader, Read};

#[derive(Debug, PartialEq)]
pub enum Format {
    Unknown(Vec<u8>),
    Big4,
    BigF,
}

impl<'a> From<&'a [u8; 4]> for Format {
    fn from(bytes: &'a [u8; 4]) -> Self {
        match *bytes {
            [66, 73, 71, 52] => Format::Big4,
            // [66, 73, 71, _] => Format::BigF,
            _ => Format::Unknown(Vec::from(bytes as &[u8])),
        }
    }
}

pub fn read_header_format(bytes: &[u8]) -> Result<Format, std::io::Error> {
    let mut reader = BufReader::new(bytes);

    let mut magic = [0; 4];
    try!(reader.read_exact(&mut magic));

    Ok(Format::from(&magic))
}

#[cfg(test)]
mod tests {
    const TEST_BYTES: &'static [u8] = include_bytes!("../test.big");

    use super::{read_header_format, Format};

    #[test]
    fn is_big4() {
        assert_eq!(read_header_format(TEST_BYTES).expect("Failed to load file"),
                   Format::Big4);
    }
}
