extern crate liblitereader;

use liblitereader::{Parser, Header, BTreePageHeader};

fn main() {
    let header = Header::from_file("history.sqlite");
    println!("{:?}", header);
    let page_header = BTreePageHeader::from_file("history.sqlite");
    println!("{:?}", page_header);
}
