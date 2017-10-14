#![cfg_attr(test, feature(test))]

extern crate byteorder;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom, BufRead, BufReader};
use std::fs::File;
use std::cell::RefCell;

pub mod errors;
use errors::ReadError;

pub struct BigArchive<T: Read + Seek> {
    pub format: Format,
    pub size: u32,

    _buf_reader: RefCell<BufReader<T>>,
    _junk_len: u32,
    _entries: HashMap<String, BigEntry>,
}

impl BigArchive<File> {
    pub fn new_from_path(path: &str) -> Result<Self, ReadError> {
        let f = try!(File::open(&path));
        let br = BufReader::new(f);
        Ok(try!(BigArchive::new(br)))
    }
}

impl<T: Read + Seek> BigArchive<T> {
    pub fn new(mut data: BufReader<T>) -> Result<Self, ReadError> {
        let format = try!(read_format(&mut data));

        if let Format::Unknown(bytes) = format {
            return Err(ReadError::UnknownArchiveFormat(bytes));
        }

        let archive_len = try!(data.read_u32::<LittleEndian>());
        let num_entries = try!(data.read_u32::<BigEndian>());
        let first_offset = try!(data.read_u32::<BigEndian>());
        let mut pos = 16;

        let mut entries = HashMap::new();

        for _ in 0..num_entries {
            let offset = try!(data.read_u32::<BigEndian>());
            let data_len = try!(data.read_u32::<BigEndian>());

            let mut buf = Vec::new();
            let name_len = try!(data.read_until(b'\0', &mut buf));

            let entry = BigEntry {
                offset: offset,
                data_len: data_len,
                name: String::from_utf8_lossy(&buf[..name_len - 1]).to_string(),
            };

            entries.insert(entry.name.clone(), entry);

            let entry_metadata_len = 4 + 4 + name_len as u32;
            pos += entry_metadata_len;
        }

        let junk_len = first_offset - pos
        // Patching an off-by-one. Need to revisit and fix this properly.
        + 1;

        Ok(BigArchive {
            format: format,
            size: archive_len,
            _buf_reader: RefCell::new(data),
            _junk_len: junk_len,
            _entries: entries,
        })
    }

    pub fn junk_len(&self) -> u32 {
        self._junk_len
    }

    pub fn entry_count(&self) -> usize {
        self._entries.len()
    }

    /// TODO: Don't return owned data, instead give the caller back a slice
    pub fn read_entry(&self, entry_name: &str) -> Option<Vec<u8>> {
        if let Some(entry) = self._entries.get(entry_name) {
            let br = &mut self._buf_reader.borrow_mut();

            if br.seek(SeekFrom::Start((*entry).offset as u64)).is_err() {
                return None;
            }

            let mut buf = vec![0; entry.data_len as usize];
            if br.read_exact(&mut buf).is_err() {
                return None;
            }

            Some(buf)
        } else {
            None
        }
    }

    pub fn contains(&self, entry_name: &str) -> bool {
        self._entries.contains_key(entry_name)
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
    pub data_len: u32,
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
            b"BIGF" => Format::BigF,
            _ => Format::Unknown(Vec::from(&buf[..])),
        }
    }
}

fn read_format(data: &mut BufRead) -> Result<Format, io::Error> {
    Ok(Format::from(data))
}

#[cfg(test)]
mod tests_bytes {
    const TEST_BYTES: &'static [u8] = include_bytes!("../test.big");
    use std::io::{BufReader, Cursor};
    use super::{Format, BigArchive};

    #[test]
    fn is_big4() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        assert_eq!(archive.format, Format::Big4);
    }

    #[test]
    fn has_two_entries() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        assert_eq!(archive.get_all_entry_names().len(), 2);
    }

    #[test]
    fn contains_art_slash_image_dot_txt() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        assert!(archive.contains("art/image.txt"));
    }

    #[test]
    fn contains_data_slash_test_dot_ini() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        assert!(archive.contains("data/test.ini"));
    }

    #[test]
    fn read_entry_art_slash_image_dot_txt() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        let data = archive.read_entry("art/image.txt").unwrap();
        assert_eq!(data, vec![98, 108, 97, 98, 108, 98, 97]);
    }

    #[test]
    fn read_entry_data_slash_test_dot_ini() {
        let c = Cursor::new(TEST_BYTES);
        let br = BufReader::new(c);
        let archive = BigArchive::new(br).unwrap();
        let data = archive.read_entry("data/test.ini").unwrap();
        assert_eq!(data,
                   vec![84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 105, 109, 112, 108,
                        101, 32, 116, 101, 115, 116, 32, 102, 105, 108, 101]);
    }

    mod benches {
        extern crate test;
        use self::test::Bencher;

        #[bench]
        fn is_big4(b: &mut Bencher) {
            b.iter(|| super::is_big4());
        }

        #[bench]
        fn has_two_entries(b: &mut Bencher) {
            b.iter(|| super::has_two_entries());
        }

        #[bench]
        fn contains_art_slash_image_dot_txt(b: &mut Bencher) {
            b.iter(|| super::contains_art_slash_image_dot_txt());
        }

        #[bench]
        fn contains_data_slash_test_dot_ini(b: &mut Bencher) {
            b.iter(|| super::contains_data_slash_test_dot_ini());
        }

        #[bench]
        fn read_entry_art_slash_image_dot_txt(b: &mut Bencher) {
            b.iter(|| super::read_entry_art_slash_image_dot_txt());
        }

        #[bench]
        fn read_entry_data_slash_test_dot_ini(b: &mut Bencher) {
            b.iter(|| super::read_entry_data_slash_test_dot_ini());
        }
    }
}

#[cfg(test)]
mod tests_file {
    use super::{Format, BigArchive};
    const ARCHIVE_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/test.big");

    #[test]
    fn is_big4() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        assert_eq!(archive.format, Format::Big4);
    }

    #[test]
    fn has_two_entries() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        assert_eq!(archive.get_all_entry_names().len(), 2);
    }

    #[test]
    fn contains_art_slash_image_dot_txt() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        assert!(archive.contains("art/image.txt"));
    }

    #[test]
    fn contains_data_slash_test_dot_ini() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        assert!(archive.contains("data/test.ini"));
    }

    #[test]
    fn read_entry_art_slash_image_dot_txt() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        let data = archive.read_entry("art/image.txt").unwrap();
        assert_eq!(data, vec![98, 108, 97, 98, 108, 98, 97]);
    }

    #[test]
    fn read_entry_data_slash_test_dot_ini() {
        let archive = BigArchive::new_from_path(&ARCHIVE_PATH).unwrap();
        let data = archive.read_entry("data/test.ini").unwrap();
        assert_eq!(data,
                   vec![84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 105, 109, 112, 108,
                        101, 32, 116, 101, 115, 116, 32, 102, 105, 108, 101]);
    }

    mod benches {
        extern crate test;
        use self::test::Bencher;

        #[bench]
        fn is_big4(b: &mut Bencher) {
            b.iter(|| super::is_big4());
        }

        #[bench]
        fn has_two_entries(b: &mut Bencher) {
            b.iter(|| super::has_two_entries());
        }

        #[bench]
        fn contains_art_slash_image_dot_txt(b: &mut Bencher) {
            b.iter(|| super::contains_art_slash_image_dot_txt());
        }

        #[bench]
        fn contains_data_slash_test_dot_ini(b: &mut Bencher) {
            b.iter(|| super::contains_data_slash_test_dot_ini());
        }

        #[bench]
        fn read_entry_art_slash_image_dot_txt(b: &mut Bencher) {
            b.iter(|| super::read_entry_art_slash_image_dot_txt());
        }

        #[bench]
        fn read_entry_data_slash_test_dot_ini(b: &mut Bencher) {
            b.iter(|| super::read_entry_data_slash_test_dot_ini());
        }
    }
}
