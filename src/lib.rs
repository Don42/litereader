#[macro_use]
extern crate nom;

pub mod parser;
pub mod data_structures;
mod enums;

use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use std::error::Error;

pub use data_structures::Header;


pub trait Sqlite<T> {
    fn from_file(path: &str) -> T;
    fn from_vec(buffer: &Vec<u8>) -> T;
    fn is_valid(&self) -> bool;
}

impl Sqlite<Header> for Header {
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
