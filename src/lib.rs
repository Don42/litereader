extern crate byteorder;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

const PAGE_SIZE_MAX: u32 = 65536;

pub struct SqliteHeader {
    hdr: Vec<u8>,
}

pub enum WriteVersion {
    Legacy,
    WAL,
}

impl std::fmt::Display for WriteVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &WriteVersion::Legacy => "Legacy",
                   &WriteVersion::WAL => "WAL",
               })
    }
}

pub trait Sqlite {
    fn new(path: &str) -> SqliteHeader;
    fn get_magic_string(&self) -> &str;
    fn get_page_size(&self) -> u32;
    fn get_write_version(&self) -> WriteVersion;
}

impl Sqlite for SqliteHeader {
    fn new(path: &str) -> SqliteHeader {
        let path = Path::new(path);

        let file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open file {}", why.description()),
            Ok(file) => file,
        };

        let mut buffer = Vec::<u8>::with_capacity(100);
        let _ = file.take(100).read_to_end(&mut buffer);
        SqliteHeader { hdr: buffer.clone() }
    }

    fn get_magic_string(&self) -> &str {
        let magic = std::str::from_utf8(&self.hdr[0..16]).unwrap();
        assert_eq!(magic, "SQLite format 3\0");
        magic
    }

    fn get_page_size(&self) -> u32 {
        let mut cur = Cursor::new(&self.hdr[16..18]);
        let size = cur.read_u16::<BigEndian>().unwrap();
        assert!(size.is_power_of_two());
        if size == 1 {
            PAGE_SIZE_MAX
        } else {
            size as u32
        }
    }

    fn get_write_version(&self) -> WriteVersion {
        match self.hdr[18] {
            1 => WriteVersion::Legacy,
            2 => WriteVersion::WAL,
            _ => panic!("Unknown WriteVersion"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
