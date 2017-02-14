extern crate liblitereader;

use std::env;

use liblitereader::{Parser, SqliteFile};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = SqliteFile::from_file(&args[1]);
    println!("{:?}", file);
}
