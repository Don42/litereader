extern crate liblitereader;

use liblitereader::{Parser, SqliteFile};

fn main() {
    let file = SqliteFile::from_file("history.sqlite");
    println!("{:?}", file);
}
