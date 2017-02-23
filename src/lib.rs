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


// TODO: Only parse header and provide access functions for the rest.
pub trait Parser<T> {
    fn from_file(path: &str) -> T;
    fn from_vec(buffer: &Vec<u8>) -> T;
    fn is_valid(&self) -> bool;
}


impl Parser<SqliteFile> for SqliteFile {
    fn from_file(path: &str) -> SqliteFile {
        let path = Path::new(path);

        println!("Path: {:?}", path);
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
