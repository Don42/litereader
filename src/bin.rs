extern crate liblitereader;

use liblitereader::{SqliteHeader, Sqlite};

fn main() {
    let header = SqliteHeader::from_file("history.sqlite");
    println!("Header: \n{}", header);
}
