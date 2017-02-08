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
    pub magic_string: String,
    pub page_size: u32,
    pub read_version: ReadVersion,
    pub write_version: WriteVersion,
	pub reserved_space: u8,
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

pub enum ReadVersion {
    Legacy,
    WAL,
}

impl std::fmt::Display for ReadVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &ReadVersion::Legacy => "Legacy",
                   &ReadVersion::WAL => "WAL",
               })
    }
}

pub trait Sqlite {
    fn from_file(path: &str) -> SqliteHeader;
}

impl std::fmt::Display for SqliteHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
"Page Size: {}
Read Version: {}
Write Version: {}
Reserved Space: {}
",
		self.page_size,
		self.read_version,
		self.write_version,
		self.reserved_space,
		)
	}
}

impl Sqlite for SqliteHeader {
    fn from_file(path: &str) -> SqliteHeader {
        let path = Path::new(path);

        let file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open file {}", why.description()),
            Ok(file) => file,
        };

        let mut buffer = Vec::<u8>::with_capacity(100);
        let count = match file.take(100).read_to_end(&mut buffer) {
            Ok(n) => n,
            Err(why) => panic!("couldn't read header {}", why.description()),
        };
        assert_eq!(count, 100);
        // Parse everything here
        let magic = std::str::from_utf8(&buffer[0..16]).unwrap();
        assert_eq!(magic, "SQLite format 3\0");
        SqliteHeader {
            hdr: buffer.clone(),
            magic_string: magic.to_string(),
            page_size: get_page_size(&buffer),
            read_version: get_read_version(&buffer),
            write_version: get_write_version(&buffer),
			reserved_space: buffer[20],
        }
    }
}

fn get_page_size(buffer: &[u8]) -> u32 {
    let mut cur = Cursor::new(&buffer[16..18]);
    let size = cur.read_u16::<BigEndian>().unwrap();
    assert!(size.is_power_of_two());
    if size == 1 {
        PAGE_SIZE_MAX
    } else {
        size as u32
    }
}

fn get_write_version(buffer: &[u8]) -> WriteVersion {
    match buffer[18] {
        1 => WriteVersion::Legacy,
        2 => WriteVersion::WAL,
        _ => panic!("Unknown WriteVersion"),
    }
}

fn get_read_version(buffer: &[u8]) -> ReadVersion {
    match buffer[19] {
        1 => ReadVersion::Legacy,
        2 => ReadVersion::WAL,
        _ => panic!("Unknown WriteVersion"),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
