extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct BigArchive {
    pub format: Format,
    pub size: u32,

    _entries: HashMap<String, BigEntry>,
}

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

impl BigArchive {
    pub fn new(mut data: &mut BufRead) -> Result<BigArchive, ReadError> {
        let format = read_format(&mut data).expect("Failed to read format");

        if let Format::Unknown(bytes) = format {
            return Err(ReadError::UnknownFormat(bytes));
        }

        let size = invert_endianness(data.read_u32::<LittleEndian>().expect("Failed to read size"));
        let num_entries = invert_endianness(data.read_u32::<LittleEndian>()
            .expect("Failed to read num_entries"));
        let _first = data.read_u32::<LittleEndian>();

        let mut entries = HashMap::new();

        for i in 0..num_entries {
            let entry_num = i + 1;
            let offset = invert_endianness(data.read_u32::<LittleEndian>()
                .expect(&format!("Failed to read offset of entry {}", entry_num)));
            let size = invert_endianness(data.read_u32::<LittleEndian>()
                .expect(&format!("Failed to read size of entry {}", entry_num)));

            let mut buf = Vec::new();
            data.read_until(b'\0', &mut buf)
                .expect(&format!("Failed to read name for entry {}", entry_num));

            let name = String::from_utf8_lossy(&buf[..buf.len() - 1]);

            let entry = BigEntry {
                offset: offset,
                size: size,
                name: name.to_string(),
            };

            entries.insert(entry.name.clone(), entry);
        }

        Ok(BigArchive {
            format: format,
            size: size,
            _entries: entries,
        })
    }

    pub fn get_entry(&self, entry_name: &str) -> Option<&BigEntry> {
        self._entries.get(entry_name)
    }

    pub fn get_entry_mut(&mut self, entry_name: &str) -> Option<&mut BigEntry> {
        self._entries.get_mut(entry_name)
    }

    pub fn get_all_entry_names(&self) -> std::collections::hash_map::Keys<String, BigEntry> {
        self._entries.keys()
    }
}

#[derive(Debug)]
pub struct BigEntry {
    pub offset: u32,
    pub size: u32,
    pub name: String,
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

fn read_format(data: &mut BufRead) -> Result<Format, io::Error> {
    Ok(Format::from(data))
}

fn invert_endianness(v: u32) -> u32 {
    (v << 24) | (v << 8 & 0xff0000) | (v >> 8 & 0xff00) | (v >> 24)
}

#[cfg(test)]
mod tests {
    const TEST_BYTES: &'static [u8] = include_bytes!("../test.big");

    use std::io::BufReader;
    use super::{read_format, Format, BigArchive};

    #[test]
    fn is_big4() {
        let mut reader = BufReader::new(TEST_BYTES);
        assert_eq!(read_format(&mut reader).expect("Failed to load file"),
                   Format::Big4);
    }

    #[test]
    fn has_two_entries() {
        let mut reader = BufReader::new(TEST_BYTES);
        let archive = BigArchive::new(&mut reader).unwrap();
        assert_eq!(archive.get_all_entry_names().len(), 2);
    }
}
