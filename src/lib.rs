#[macro_use]
extern crate nom;

pub mod parser;
pub mod data_structures;
mod enums;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;

pub use data_structures::{Header, BTreePageHeader, SqliteFile};


pub trait Parser<T> {
    fn from_file(path: &str) -> T;
    fn from_vec(buffer: &Vec<u8>) -> T;
    fn is_valid(&self) -> bool;
}

impl Parser<Header> for Header {
    fn from_file(path: &str) -> Header {
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
        Header::from_vec(&buffer)
    }

    fn from_vec(buffer: &Vec<u8>) -> Header {
        parser::parse_header(&buffer).unwrap()
    }

    fn is_valid(&self) -> bool {
        self.max_embedded_payload_fraction == 64 && self.min_embedded_payload_fraction == 32
    }
}

impl Parser<SqliteFile> for SqliteFile {
    fn from_file(path: &str) -> SqliteFile {
        let path = Path::new(path);

        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open file {}", why.description()),
            Ok(file) => file,
        };

        let mut buffer = Vec::<u8>::with_capacity(100);
        let count = match file.read_to_end(&mut buffer) {
            Ok(n) => n,
            Err(why) => panic!("couldn't read header {}", why.description()),
        };
        assert!(count > 100);
        SqliteFile::from_vec(&buffer)
    }

    fn from_vec(buffer: &Vec<u8>) -> SqliteFile {
        parser::parse_sqlite_file(&buffer).unwrap()
    }

    fn is_valid(&self) -> bool {
        unimplemented!()
    }
}

impl Parser<BTreePageHeader> for BTreePageHeader {
    fn from_file(path: &str) -> BTreePageHeader {
        let path = Path::new(path);

        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open file {}", why.description()),
            Ok(file) => file,
        };

        let mut buffer = Vec::<u8>::with_capacity(100);
        let count = match file.read_to_end(&mut buffer) {
            Ok(n) => n,
            Err(why) => panic!("couldn't read header {}", why.description()),
        };
        assert!(count > 100);
        BTreePageHeader::from_vec(&buffer)
    }

    fn from_vec(buffer: &Vec<u8>) -> BTreePageHeader {
        parser::parse_btree_page_header(&buffer[100..]).unwrap()
    }

    fn is_valid(&self) -> bool {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
