extern crate liblitereader;

use liblitereader::{Sqlite};

fn main() {
    let header = liblitereader::parser::Header::from_file("history.sqlite");
    println!("{:?}", header);
}
