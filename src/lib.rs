extern crate byteorder;

mod enums;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use enums::{ReadVersion, WriteVersion, TextEncoding, SchemaFormat,
            get_schema_format, get_text_encoding, get_write_version, get_read_version};

const HEADER_STRING: &'static str = "SQLite format 3\0";
const PAGE_SIZE_MAX: u32 = 65536;


pub trait Sqlite {
    fn from_file(path: &str) -> SqliteHeader;
    fn from_vec(buffer: Vec<u8>) -> SqliteHeader;
    fn is_valid(&self) -> bool;
}


pub struct SqliteHeader {
    hdr: Vec<u8>,
    pub magic_string: String,
    pub page_size: u32,
    pub read_version: ReadVersion,
    pub write_version: WriteVersion,
    pub reserved_space: u8,
    pub max_embedded_payload_fraction: u8,
    pub min_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8,
    pub file_change_counter: u32,
    pub database_size: u32,
    pub freelist_trunk_page: u32,
    pub freelist_count: u32,
    pub schema_cookie: u32,
    pub schema_format: SchemaFormat,
    pub default_page_cache_size: u32,
    pub largest_root_page: u32,
    pub text_encoding: TextEncoding,
    pub user_version: u32,
    pub incremental_vacuum_mode: bool,
    pub application_id: u32,
    pub reserved_area: [u8; 20],
    pub version_valid_for: u32,
    pub sqlite_version_number: u32,
}

impl std::fmt::Display for SqliteHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Page Size: {}
Read Version: {}
Write Version: {}
Reserved Space: {}
Max Embedded Payload Fraction: {}
Min Embedded Payload Fraction: {}
Leaf Payload Fraction: {}
File Change Counter: {}
Database Size: {}
Freelist Trunk Page: {}
Freelist Count: {}
Schema Cookie: {}
Schema Format Number: {}
Default Page Cache Size: {}
Largest Root Page: {}
Text Encoding: {}
User Version: {}
Incremental Vacuum Mode: {}
Application ID: {}
Version Valid For: {}
SQLite Version Number: {}
Valid: {}
",
               self.page_size,
               self.read_version,
               self.write_version,
               self.reserved_space,
               self.max_embedded_payload_fraction,
               self.min_embedded_payload_fraction,
               self.leaf_payload_fraction,
               self.file_change_counter,
               self.database_size,
               self.freelist_trunk_page,
               self.freelist_count,
               self.schema_cookie,
               self.schema_format,
               self.default_page_cache_size,
               self.largest_root_page,
               self.text_encoding,
               self.user_version,
               self.incremental_vacuum_mode,
               self.application_id,
               self.version_valid_for,
               self.sqlite_version_number,
               self.is_valid(),
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
        SqliteHeader::from_vec(buffer)
    }

    fn from_vec(buffer: Vec<u8>) -> SqliteHeader {
        // Parse everything here
        let magic = std::str::from_utf8(&buffer[0..16]).unwrap();
        assert_eq!(magic, HEADER_STRING);
        let mut cur = Cursor::new(&buffer[16..]);
        SqliteHeader {
            hdr: buffer.clone(),
            magic_string: magic.to_string(),
            page_size: get_page_size(cur.read_u16::<BigEndian>().unwrap()),
            read_version: get_read_version(cur.read_u8().unwrap()).unwrap(),
            write_version: get_write_version(cur.read_u8().unwrap()).unwrap(),
            reserved_space: cur.read_u8().unwrap(),
            max_embedded_payload_fraction: cur.read_u8().unwrap(),
            min_embedded_payload_fraction: cur.read_u8().unwrap(),
            leaf_payload_fraction: cur.read_u8().unwrap(),
            file_change_counter: cur.read_u32::<BigEndian>().unwrap(),
            database_size: cur.read_u32::<BigEndian>().unwrap(),
            freelist_trunk_page: cur.read_u32::<BigEndian>().unwrap(),
            freelist_count: cur.read_u32::<BigEndian>().unwrap(),
            schema_cookie: cur.read_u32::<BigEndian>().unwrap(),
            schema_format: get_schema_format(cur.read_u32::<BigEndian>().unwrap()).unwrap(),
            default_page_cache_size: cur.read_u32::<BigEndian>().unwrap(),
            largest_root_page: cur.read_u32::<BigEndian>().unwrap(),
            text_encoding: get_text_encoding(cur.read_u32::<BigEndian>().unwrap()).unwrap(),
            user_version: cur.read_u32::<BigEndian>().unwrap(),
            incremental_vacuum_mode: match cur.read_u32::<BigEndian>().unwrap() {
                0 => false,
                1 => true,
                x => panic!("Unknown incremental vacuum option: {}", x),
            },
            application_id: cur.read_u32::<BigEndian>().unwrap(),
            reserved_area: {
                let mut reserved_buffer = [0; 20];
                cur.read_exact(&mut reserved_buffer).unwrap();
                reserved_buffer
            },
            version_valid_for: cur.read_u32::<BigEndian>().unwrap(),
            sqlite_version_number: cur.read_u32::<BigEndian>().unwrap(),
        }
    }

    fn is_valid(&self) -> bool {
        self.magic_string == HEADER_STRING && self.max_embedded_payload_fraction == 64 &&
            self.min_embedded_payload_fraction == 32
    }
}


fn get_page_size(size: u16) -> u32 {
    assert!(size.is_power_of_two());
    if size == 1 {
        PAGE_SIZE_MAX
    } else {
        size as u32
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
