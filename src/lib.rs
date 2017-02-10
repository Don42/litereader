#[macro_use]
extern crate nom;

pub mod parser;
mod enums;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;

use enums::{ReadVersion, WriteVersion, TextEncoding, SchemaFormat};


pub trait Sqlite<T> {
    fn from_file(path: &str) -> T;
    fn from_vec(buffer: Vec<u8>) -> T;
    fn is_valid(&self) -> bool;
}


pub struct SqliteHeader {
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

impl Sqlite<parser::Header> for parser::Header {
    fn from_file(path: &str) -> parser::Header {
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
        parser::parse_header(&buffer).unwrap()

    }

    fn from_vec(buffer: Vec<u8>) -> parser::Header {
        parser::parse_header(&buffer).unwrap()
    }

    fn is_valid(&self) -> bool {
        true
        //self.magic_string == HEADER_STRING && self.max_embedded_payload_fraction == 64 &&
        //    self.min_embedded_payload_fraction == 32
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
