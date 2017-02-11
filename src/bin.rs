extern crate liblitereader;

use liblitereader::{Sqlite, Header};

fn main() {
    let header = Header::from_file("history.sqlite");
    println!("{:?}", header);
}
