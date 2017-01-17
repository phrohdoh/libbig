extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{BufRead, Read};
use std::fs::File;

#[derive(Debug)]
pub struct BigArchive {
    pub format: Format,
    pub size: u32,
}

impl BigArchive {
    pub fn new(mut data: &mut BufRead) -> Result<BigArchive, std::io::Error> {
        let format = read_format(&mut data).expect("Failed to read format");
        let size = data.read_u32::<LittleEndian>().expect("Failed to read size");
        let num_entries = data.read_u32::<LittleEndian>().expect("Failed to read num_entries");
        let _first = data.read_u32::<LittleEndian>();

        for i in 0..num_entries {
            let offset = data.read_u32::<LittleEndian>()
                .expect(&format!("Failed to read offset of entry {}", i + 1));
            let size = data.read_u32::<LittleEndian>()
                .expect(&format!("Failed to read size of entry {}", i + 1));

            let mut buf = Vec::new();
            data.read_until(b'\0', &mut buf)
                .expect(&format!("Failed to read name for entry {}", i + 1));
        }

        Ok(BigArchive {
            format: format,
            size: size,
        })
    }
}

pub struct BigEntry<'archive> {
    pub offset: u32,
    pub size: u32,
    pub name: &'archive str,
}

#[derive(Debug, PartialEq)]
pub enum Format {
    Unknown(Vec<u8>),
    Big4,
    BigF,
}

impl<'a> From<&'a mut BufRead> for Format {
    fn from(data: &'a mut BufRead) -> Self {
        let mut buf = [0; 4];
        let _ = data.read_exact(&mut buf);
        match &buf {
            b"BIG4" => Format::Big4,
            b"BIGf" => Format::BigF,
            _ => Format::Unknown(Vec::from(&buf[..])),
        }
    }
}

fn read_format(data: &mut BufRead) -> Result<Format, std::io::Error> {
    Ok(Format::from(data))
}

#[cfg(test)]
mod tests {
    const TEST_BYTES: &'static [u8] = include_bytes!("../test.big");

    use super::{read_format, Format};

    #[test]
    fn is_big4() {
        assert_eq!(read_format(TEST_BYTES).expect("Failed to load file"),
                   Format::Big4);
    }
}
