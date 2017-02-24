#[macro_use]
extern crate nom;

pub mod parser;
pub mod data_structures;
mod enums;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;

use nom::{ErrorKind, IResult};

pub use data_structures::{Header, BTreePageHeader, BTreePage, SqliteFile};
use parser::header_parser;


// TODO: Only parse header and provide access functions for the rest.
// Doesn't need to be generic
pub trait Parser<T> {
    fn from_file(path: &str) -> Result<T, String>;
    fn from_vec(buffer: &Vec<u8>) -> Result<T, String>;
    fn is_valid(&self) -> bool;
    fn get_page(&self, usize) -> Result<BTreePage, String>;
}


impl Parser<SqliteFile> for SqliteFile {
    fn from_file(path: &str) -> Result<SqliteFile, String> {
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

    fn from_vec(buffer: &Vec<u8>) -> Result<SqliteFile, String> {
        let (_, file_header) = match header_parser(buffer) {
            IResult::Done(x, y) => (x, y),
            IResult::Error(ErrorKind::Tag) => {
                return Err("File is not SQLite Database".to_string())
            },
            IResult::Error(x) => {
                println!("{:?}", x);
                return Err("Error parsing header".to_string())
            },
            IResult::Incomplete(_) => { return Err("Incomplete header".to_string()) },
        };

        Ok(SqliteFile::new(file_header, buffer.to_vec()))
    }

    fn is_valid(&self) -> bool {
        unimplemented!()
    }

    fn get_page(&self, page_id: usize) -> Result<BTreePage, String> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
