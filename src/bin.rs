extern crate liblitereader;

use liblitereader::{SqliteHeader, Sqlite};

fn main() {
    let header = SqliteHeader::new("history.sqlite");
    println!("Magic String: {}", header.get_magic_string());
    println!("Page size: {}", header.get_page_size());
    println!("Write version: {}", header.get_write_version());
}
